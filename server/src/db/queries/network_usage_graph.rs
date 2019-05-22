use common_rpc_types::{
    GraphCommonArgs, GraphNetworkOutAllRow, GraphNetworkOutNodeExtensionRow, GraphNetworkOutNodePeerRow, GraphPeriod,
    NodeName,
};
use postgres;

pub fn query_network_out_all(
    conn: &postgres::Connection,
    graph_args: GraphCommonArgs,
) -> postgres::Result<Vec<GraphNetworkOutAllRow>> {
    let time_column_name = get_sql_column_name_by_period(graph_args.period);
    let query_stmt = format!(
        "\
         SELECT \
         name, \
         {}, \
         CAST (SUM(bytes) AS REAL) as value \
         FROM \"network_usage\" \
         WHERE \"{}\"<$1 and \"{}\">$2 \
         GROUP BY \"name\", \"{}\" \
         ORDER BY \"name\", \"{}\" ASC",
        time_column_name, time_column_name, time_column_name, time_column_name, time_column_name
    );

    let rows = conn.query(&query_stmt, &[&graph_args.to, &graph_args.from])?;

    Ok(rows
        .into_iter()
        .map(|row| GraphNetworkOutAllRow {
            node_name: row.get("name"),
            time: row.get(time_column_name),
            value: row.get("value"),
        })
        .collect())
}

fn get_sql_column_name_by_period(period: GraphPeriod) -> &'static str {
    match period {
        GraphPeriod::Minutes5 => "time_5min",
        GraphPeriod::Hour => "time_hour",
        GraphPeriod::Day => "time_day",
    }
}

pub fn query_network_out_all_avg(
    conn: &postgres::Connection,
    graph_args: GraphCommonArgs,
) -> postgres::Result<Vec<GraphNetworkOutAllRow>> {
    let time_column_name = get_sql_column_name_by_period(graph_args.period);
    let query_stmt = format!(
        "\
         SELECT \
         \"network_usage\".name, \
         {}, \
         CAST (SUM(bytes/\"peer_count\".\"peer_count\") AS REAL) as value \
         FROM \"network_usage\" \
         LEFT JOIN peer_count ON (\"network_usage\".\"time\"=\"peer_count\".\"time\" AND \
         \"network_usage\".\"name\"=\"peer_count\".\"name\") \
         WHERE \"network_usage\".\"{}\"<$1 and \"network_usage\".\"{}\">$2 \
         GROUP BY \"network_usage\".\"name\", \"{}\" \
         ORDER BY \"network_usage\".\"name\", \"{}\" ASC",
        time_column_name, time_column_name, time_column_name, time_column_name, time_column_name
    );

    let rows = conn.query(&query_stmt, &[&graph_args.to, &graph_args.from])?;

    Ok(rows
        .into_iter()
        .map(|row| GraphNetworkOutAllRow {
            node_name: row.get("name"),
            time: row.get(time_column_name),
            value: row.get("value"),
        })
        .collect())
}

pub fn query_network_out_node_extension(
    conn: &postgres::Connection,
    node_name: NodeName,
    graph_args: GraphCommonArgs,
) -> postgres::Result<Vec<GraphNetworkOutNodeExtensionRow>> {
    let time_column_name = get_sql_column_name_by_period(graph_args.period);
    let query_stmt = format!(
        "\
         SELECT \
         extension, \
         {}, \
         CAST (SUM(bytes) AS REAL) as value \
         FROM \"network_usage\" \
         WHERE \"network_usage\".\"{}\"<$1 AND \"network_usage\".\"{}\">$2 \
           AND \"network_usage\".\"name\"=$3
         GROUP BY \"network_usage\".\"extension\", \"{}\" \
         ORDER BY \"network_usage\".\"extension\", \"{}\" ASC",
        time_column_name, time_column_name, time_column_name, time_column_name, time_column_name
    );

    let rows = conn.query(&query_stmt, &[&graph_args.to, &graph_args.from, &node_name])?;

    Ok(rows
        .into_iter()
        .map(|row| GraphNetworkOutNodeExtensionRow {
            extension: row.get("extension"),
            time: row.get(time_column_name),
            value: row.get("value"),
        })
        .collect())
}

pub fn query_network_out_node_peer(
    conn: &postgres::Connection,
    node_name: NodeName,
    graph_args: GraphCommonArgs,
) -> postgres::Result<Vec<GraphNetworkOutNodePeerRow>> {
    let time_column_name = get_sql_column_name_by_period(graph_args.period);
    let query_stmt = format!(
        "\
         SELECT \
         \"target_ip\", \
         {}, \
         CAST (SUM(bytes) AS REAL) as value \
         FROM \"network_usage\" \
         WHERE \"network_usage\".\"{}\"<$1 AND \"network_usage\".\"{}\">$2 \
           AND \"network_usage\".\"name\"=$3
         GROUP BY \"network_usage\".\"target_ip\", \"{}\" \
         ORDER BY \"network_usage\".\"target_ip\", \"{}\" ASC",
        time_column_name, time_column_name, time_column_name, time_column_name, time_column_name
    );

    let rows = conn.query(&query_stmt, &[&graph_args.to, &graph_args.from, &node_name])?;

    Ok(rows
        .into_iter()
        .map(|row| GraphNetworkOutNodePeerRow {
            peer: row.get("target_ip"),
            time: row.get(time_column_name),
            value: row.get("value"),
        })
        .collect())
}
