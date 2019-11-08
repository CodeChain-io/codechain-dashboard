use chrono;
use postgres;
use regex::{Captures, Regex};

use super::super::types::DBConnection;
use common_rpc_types::NetworkUsage;
use util::{floor_to_5min, start_of_day, start_of_hour};

pub fn insert(
    conn: &DBConnection,
    node_name: &str,
    network_usage: NetworkUsage,
    time: chrono::DateTime<chrono::Utc>,
) -> postgres::Result<()> {
    ctrace!("Add network usage of {}", node_name);

    if network_usage.is_empty() {
        return Ok(())
    }

    let stmt = conn.prepare(
        "INSERT INTO network_usage (time, name, extension, target_ip, bytes, time_5min, time_hour, time_day) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )?;
    for key in network_usage.keys() {
        let parse_result = parse_network_usage_key(key);
        let (extension, ip) = match parse_result {
            Ok((extension, ip)) => (extension, ip),
            Err(err) => {
                cerror!("Network Usage Parse Failed {:?}", err);
                // FIXME: propagate the error
                return Ok(())
            }
        };
        let bytes = network_usage[key];

        stmt.execute(&[
            &time,
            &node_name,
            &extension,
            &ip,
            &bytes,
            &floor_to_5min(&time),
            &start_of_hour(&time),
            &start_of_day(&time),
        ])?;
    }

    Ok(())
}

fn parse_network_usage_key(key: &str) -> Result<(String, String), String> {
    // Ex) ::block-propagation@54.180.74.243:3485
    lazy_static! {
        static ref KEY_REGEX: Regex = Regex::new(r"::(?P<extension>[a-zA-Z\-]*)@(?P<ip>[0-9\.]*)").unwrap();
    }

    let reg_result: Captures = KEY_REGEX.captures(key).ok_or_else(|| "Parse Error".to_string())?;

    Ok((reg_result["extension"].to_string(), reg_result["ip"].to_string()))
}

pub fn remove_older_logs(conn: &postgres::Connection, time: chrono::DateTime<chrono::Utc>) -> postgres::Result<()> {
    ctrace!("Remove network usage older than {}", time);

    let result = conn.execute("DELETE FROM network_usage WHERE time<$1", &[&time])?;
    ctrace!("Delete result {}", result);
    Ok(())
}
