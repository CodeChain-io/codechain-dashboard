use std::borrow::Borrow;
use std::rc::Rc;

use chrono;
use postgres;
use postgres::types::ToSql;

use super::super::super::common_rpc_types::{NodeName, StructuredLog};
use super::super::types::OrderBy;
use super::super::types::{Log, LogQueryParams};

pub fn insert(conn: &postgres::Connection, node_name: &NodeName, logs: Vec<StructuredLog>) -> postgres::Result<()> {
    ctrace!("Add log {} : {:?}", node_name, logs);

    if logs.len() == 0 {
        return Ok(())
    }

    for log_chunk in logs.chunks(1000) {
        let mut parameters_positions: Vec<String> = Vec::new();
        let mut parameters: Vec<Box<ToSql>> = Vec::new();

        for (row_index, log) in log_chunk.into_iter().enumerate() {
            let base_num = row_index * 6;
            parameters_positions.push(format!(
                "(${}, ${}, ${}, ${}, ${}, ${})",
                base_num + 1,
                base_num + 2,
                base_num + 3,
                base_num + 4,
                base_num + 5,
                base_num + 6
            ));
            let rfc3339with_nano_second = "%Y-%m-%dT%H:%M:%S.%f%z";
            let datetime = chrono::DateTime::parse_from_str(&log.timestamp, rfc3339with_nano_second).unwrap();
            parameters.push(Box::new(node_name));
            parameters.push(Box::new(log.level.clone()));
            parameters.push(Box::new(log.target.clone()));
            parameters.push(Box::new(log.message.clone()));
            parameters.push(Box::new(datetime));
            parameters.push(Box::new(log.thread_name.clone()));
        }

        let full_sql = format!(
            "INSERT INTO logs (name, level, target, message, timestamp, thread_name) VALUES {}",
            parameters_positions.join(", ")
        );
        let parameters_ref: Vec<&ToSql> = parameters.iter().map(|param| param.as_ref()).collect();
        ctrace!("Full query is {}", full_sql);
        conn.execute(&full_sql, &parameters_ref)?;
    }

    Ok(())
}

pub fn search(conn: &postgres::Connection, params: LogQueryParams) -> postgres::Result<Vec<Log>> {
    ctrace!("Search log with {:?}", params);
    let mut parameters = Parameters::new();
    let mut where_conditions = Vec::new();
    if let Some(filter) = params.filter {
        if filter.node_names.len() != 0 {
            let node_names_index = parameters.add(Rc::new(filter.node_names));
            where_conditions.push(format!("name = ANY(${})", node_names_index));
        }
        if filter.levels.len() != 0 {
            let uppercase_levels: Vec<String> =
                filter.levels.iter().map(|level| level.to_string().to_uppercase()).collect();
            let filters_index = parameters.add(Rc::new(uppercase_levels));
            where_conditions.push(format!("level = ANY(${})", filters_index));
        }
        if filter.targets.len() != 0 {
            let targets_index = parameters.add(Rc::new(filter.targets));
            where_conditions.push(format!("target = ANY(${})", targets_index));
        }
        if let Some(thread_name) = filter.thread_name {
            let target_index = parameters.add(Rc::new(thread_name));
            where_conditions.push(format!("thread_name = ${}", target_index));
        }
    }
    if let Some(search) = params.search {
        if search != "" {
            let search_index = parameters.add(Rc::new(format!("%{}%", search)));
            where_conditions.push(format!("message ILIKE ${}", search_index));
        }
    }
    if let Some(time) = params.time {
        if let Some(from) = time.from_time {
            let from_index = parameters.add(Rc::new(from));
            where_conditions.push(format!("timestamp > ${}", from_index));
        }
        if let Some(to) = time.to_time {
            let to_index = parameters.add(Rc::new(to));
            where_conditions.push(format!("timestamp < ${}", to_index));
        }
    }

    let where_clause = if where_conditions.len() > 0 {
        "WHERE ".to_string() + &where_conditions.join(" AND ")
    } else {
        "".to_string()
    };

    let order_by = params.order_by.unwrap_or(OrderBy::ASC);
    let order_by_clause = format!("ORDER BY timestamp {:?}", order_by);

    let limit = params.item_per_page.unwrap_or(100);
    let limit_clause = format!("LIMIT {}", limit);

    // page starts from 1
    let offset = params.page.unwrap_or(1) - 1;
    let offset_clause = format!("OFFSET {}", offset * limit);

    let query_string =
        vec!["SELECT * FROM logs", &where_clause, &order_by_clause, &limit_clause, &offset_clause].join(" ");

    let query_params: Vec<&ToSql> = parameters.get().iter().map(|param| param.borrow()).collect();
    let rows = conn.query(&query_string, &query_params[..])?;

    Ok(rows
        .into_iter()
        .map(|row| Log {
            id: row.get("id"),
            node_name: row.get("name"),
            level: row.get("level"),
            target: row.get("target"),
            timestamp: row.get("timestamp"),
            message: format!("{} {}", row.get::<_, String>("thread_name"), row.get::<_, String>("message")),
        })
        .collect())
}

struct Parameters {
    parameter_count: i32,
    parameters: Vec<Rc<ToSql>>,
}

impl Parameters {
    pub fn new() -> Parameters {
        Parameters {
            parameter_count: 0,
            parameters: Vec::new(),
        }
    }

    pub fn add(&mut self, param: Rc<ToSql>) -> i32 {
        self.parameters.push(param);
        self.parameter_count += 1;
        self.parameter_count
    }

    pub fn get(&self) -> &Vec<Rc<ToSql>> {
        &self.parameters
    }
}

pub fn get_targets(conn: &postgres::Connection) -> postgres::Result<Vec<String>> {
    ctrace!("Query targets");

    //    let rows = conn.query("SELECT DISTINCT target FROM logs", &[])?;
    // Below query prints the same result with above query.
    // See https://wiki.postgresql.org/wiki/Loose_indexscan
    let rows = conn.query(
        "
    WITH RECURSIVE t AS (
       (SELECT target FROM logs ORDER BY target LIMIT 1)  -- parentheses required
           UNION ALL
           SELECT (SELECT target FROM logs WHERE target > t.target ORDER BY target LIMIT 1)
           FROM t
           WHERE t.target IS NOT NULL
       )
    SELECT target FROM t WHERE target IS NOT NULL",
        &[],
    )?;
    Ok(rows.iter().map(|row| row.get("target")).collect())
}
