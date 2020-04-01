use super::super::types::DBConnection;
use crate::common_rpc_types::{
    GraphCommonArgs, GraphNetworkOutAllRow, GraphNetworkOutNodeExtensionRow, GraphNetworkOutNodePeerRow, GraphPeriod,
    NodeName,
};

pub fn query_network_out_all(
    conn: &DBConnection,
    graph_args: GraphCommonArgs,
) -> postgres::Result<Vec<GraphNetworkOutAllRow>> {
    let time_column_name = get_sql_column_name_by_period(graph_args.period);
    let query_stmt = "\
                      SELECT \
                      name, \
                      time_5min, \
                      value \
                      FROM time_5min_report_view_materialized \
                      WHERE time_5min<$1 and time_5min>$2";

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
    conn: &DBConnection,
    graph_args: GraphCommonArgs,
) -> postgres::Result<Vec<GraphNetworkOutAllRow>> {
    let time_column_name = get_sql_column_name_by_period(graph_args.period);
    let query_stmt = "\
                      SELECT \
                      name, \
                      time_5min, \
                      value \
                      FROM time_5min_avg_report_view_materialized \
                      WHERE time_5min<$1 and time_5min>$2";

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
    conn: &DBConnection,
    node_name: NodeName,
    graph_args: GraphCommonArgs,
) -> postgres::Result<Vec<GraphNetworkOutNodeExtensionRow>> {
    let time_column_name = get_sql_column_name_by_period(graph_args.period);
    let query_stmt = "\
                      SELECT \
                      extension, \
                      time_5min, \
                      value \
                      FROM time_5min_extension_report_view_materialized \
                      WHERE time_5min<$1 AND time_5min>$2 \
                      AND name=$3";

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
    conn: &DBConnection,
    node_name: NodeName,
    graph_args: GraphCommonArgs,
) -> postgres::Result<Vec<GraphNetworkOutNodePeerRow>> {
    let time_column_name = get_sql_column_name_by_period(graph_args.period);
    let query_stmt = "\
                      SELECT \
                      target_ip, \
                      time_5min, \
                      value \
                      FROM time_5min_peer_report_view_materialized \
                      WHERE time_5min<$1 AND time_5min>$2 \
                      AND name=$3";

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
