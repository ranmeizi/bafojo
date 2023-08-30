use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use toml;

pub static CFG: Lazy<Config> = Lazy::new(read_cfg);

#[derive(Deserialize)]
pub struct Config {
    pub app: AppCfg,
    pub jwt: JwtCfg,
}

#[derive(Deserialize)]
pub struct AppCfg {
    pub port: String,
    pub dbstr: String,
    pub redis_str: String,
}

#[derive(Deserialize)]
pub struct JwtCfg {
    pub exp_min: i64,
    pub secret: String,
}

fn read_cfg() -> Config {
    // 读取 Config.toml.local 文件
    let f = File::open("Config.toml.local")
        .expect("配置文件读取失败,请确认根目录下的Config.toml.local文件");
    let reader = BufReader::new(f);
    let mut buffer = String::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                buffer += &format!("{l}\r\n");
            }
            Err(_) => {}
        }
    }

    let config: Config = toml::from_str(&buffer).expect("配置文件参数类型错误");

    config
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read() {
        read_cfg();
    }

    #[test]
    fn test_port() {
        let cfg = read_cfg();

        assert_eq!(cfg.app.port, "3000");
    }

    #[test]
    fn test_cfg_port() {
        assert_eq!(CFG.app.port, "3000");
    }
}
