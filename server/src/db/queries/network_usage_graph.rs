use common_rpc_types::{GraphCommonArgs, GraphNetworkOutAllRow, GraphPeriod};
use postgres;

pub fn query_network_out_all(
    conn: &postgres::Connection,
    graph_args: GraphCommonArgs,
) -> postgres::Result<Vec<GraphNetworkOutAllRow>> {
    let query_stmt = format!(
        "\
         SELECT \
         name, \
         {}, \
         CAST (AVG(bytes) AS REAL) as value \
         FROM \"network_usage\" \
         WHERE \"time\"<$1 and \"time\">$2 \
         GROUP BY \"name\", \"rounded_time\" \
         ORDER BY \"name\", \"rounded_time\" ASC",
        get_sql_round_period_expression(graph_args.period)
    );

    let rows = conn.query(&query_stmt, &[&graph_args.to, &graph_args.from])?;

    Ok(rows
        .into_iter()
        .map(|row| GraphNetworkOutAllRow {
            node_name: row.get("name"),
            time: row.get("rounded_time"),
            value: row.get("value"),
        })
        .collect())
}

fn get_sql_round_period_expression(period: GraphPeriod) -> &'static str {
    match period {
        GraphPeriod::Minutes5 => {
            "date_trunc('hour', time) + INTERVAL '5 min' * ROUND(date_part('minute', time) / 5.0) as \"rounded_time\""
        }
        GraphPeriod::Hour => "date_trunc('hour', time) as \"rounded_time\"",
        GraphPeriod::Day => "date_trunc('day', time) as \"rounded_time\"",
    }
}
