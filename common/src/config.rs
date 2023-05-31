use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use toml;

// pub const CFG: Config = read();

#[derive(Deserialize)]
struct Config {
    app: AppCfg,
    jwt: JwtCfg,
}

#[derive(Deserialize)]
struct AppCfg {
    port: String,
    dbstr: String,
}

#[derive(Deserialize)]
struct JwtCfg {
    exp: i64,
    secret: String,
}

fn read_cfg() -> Config {
    let f = File::open("../Config.toml.local")
        .expect("配置文件读取失败,请确认根目录下的Config.toml.local文件");
    let mut reader = BufReader::new(f);
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
}
