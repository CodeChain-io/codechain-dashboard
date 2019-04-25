use chrono;
use postgres;
use regex::{Captures, Regex};

use common_rpc_types::NetworkUsage;

pub fn insert(
    conn: &postgres::Connection,
    node_name: &str,
    network_usage: NetworkUsage,
    time: chrono::DateTime<chrono::Utc>,
) -> postgres::Result<()> {
    ctrace!("Add network usage of {}", node_name);

    if network_usage.is_empty() {
        return Ok(())
    }

    let stmt = conn
        .prepare("INSERT INTO network_usage (time, name, extension, target_ip, bytes) VALUES ($1, $2, $3, $4, $5)")?;
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
        stmt.execute(&[&time, &node_name, &extension, &ip, &bytes])?;
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
