use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProxyConfig {
    id: String,
    system_type: String,
    local_env_path: String,
    proxy_url: String,
    proxy_type: String,
    status: String,
    description: String,
}

#[tauri::command]
pub fn add_proxy_config(conn: &Connection, conf: ProxyConfig) -> Result<()> {
    conn.execute(
        "INSERT INTO proxy_config VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            &conf.id,
            &conf.system_type,
            &conf.local_env_path,
            &conf.proxy_url,
            &conf.proxy_type,
            &conf.status,
            &conf.description,
        ),
    )?;

    Ok(())
}

#[tauri::command]
pub fn query_proxy_configs() -> Result<Vec<ProxyConfig>, String> {
    let conn = Connection::open("proxy-desk.db")
        .map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM proxy_config")
        .map_err(|e| e.to_string())?;

    let proxy_configs = stmt.query_map([], |row| {
        Ok(ProxyConfig {
            id: row.get(0)?,
            system_type: row.get(1)?,
            local_env_path: row.get(2)?,
            proxy_url: row.get(3)?,
            proxy_type: row.get(4)?,
            status: row.get(5)?,
            description: row.get(6)?,
        })
    })
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>>>()
    .map_err(|e| e.to_string())?;

    Ok(proxy_configs)
}

fn connect_database() -> Result<()> {
    let conn = Connection::open("proxy-desk.db")?;

    let mut stmt = conn.prepare("SELECT * FROM proxy_config")?;
    let proxy_config_iter = stmt.query_map([], |row| {
        Ok(ProxyConfig {
            id: row.get(0)?,
            system_type: row.get(1)?,
            local_env_path: row.get(2)?,
            proxy_url: row.get(3)?,
            proxy_type: row.get(4)?,
            status: row.get(5)?,
            description: row.get(6)?,
        })
    })?;

    for conf in proxy_config_iter {
        println!("Found proxy config: {:?}", conf?);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_database() -> Result<()> {
        connect_database()?;

        println!("success");
        Ok(())
    }

    #[test]
    fn test_add_proxy_config() -> Result<()>{
        let git_http_config = ProxyConfig {
            id: "uu0009".to_string(),
            system_type: "git".to_string(),
            local_env_path: "/path/to/git".to_string(),
            proxy_url: "socks5:127.0.0.1:7897".to_string(),
            proxy_type: "http".to_string(),
            status: "not Active".to_string(),
            description: "http proxy for git...".to_string(),
        };

        let conn = Connection::open("proxy-desk.db")?;
        let _ = add_proxy_config(&conn, git_http_config);

        Ok(())
    }

    #[test]
    fn test_query_proxy_configs() -> Result<()>{
        
        let confs = query_proxy_configs().unwrap();
        println!("confs: {:?}", confs);
        Ok(())
    }
}
