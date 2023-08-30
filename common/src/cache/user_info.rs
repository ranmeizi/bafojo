use super::get_conn;
use crate::dto::system::UserDto;
use anyhow::Result;
use r2d2_redis::redis::Commands;
use serde_json;

static PREFIX: &str = "UserDtoCache:";

pub async fn get(key: &str) -> Result<UserDto> {
    let mut conn = get_conn()?;
    let k = format!("{}{}", PREFIX, key);
    let v: String = conn.get(k)?;
    let user: UserDto = serde_json::from_str(&v)?;

    Ok(user)
}

pub fn set(key: &str, data: UserDto) -> () {
    if let Ok(mut conn) = get_conn() {
        let k = format!("{}{}", PREFIX, key);
        let v = serde_json::to_string(&data).unwrap();
        let _: () = conn.set(&k, v).unwrap();
    }
}
pub fn del(key: &str) -> () {
    if let Ok(mut conn) = get_conn() {
        let k = format!("{}{}", PREFIX, key);
        conn.del(k).unwrap()
    }
}

#[cfg(test)]
mod test_redis {
    use super::*;
    use tokio;

    #[test]
    fn test_set() {
        set(
            "12345",
            UserDto {
                id: 12345,
                uname: "ikun".into(),
                nickname: Some("坤坤".into()),
                sex: None,
                mobile: Some("18910830783".into()),
                email: None,
                enabled: true,
                created_at: None,
                created_by: None,
                updated_at: None,
                updated_by: None,
            },
        );
    }

    #[tokio::test]
    async fn test_get() {
        // 关闭 redis 服务 第一次超时，但第二次不应超时
        println!("开始第一次尝试");
        let res = get("12345").await;

        if let Err(e) = res {
            println!("1，失败了哦")
        } else {
            let v = res.unwrap();
            println!("1，成功了哦 {:?}", v)
        }
        println!("开始第二次尝试");
        let res = get("12345").await;

        if let Err(e) = res {
            println!("2，失败了哦")
        } else {
            let v = res.unwrap();
            println!("2，成功了哦 {:?}", v)
        }
    }
}
