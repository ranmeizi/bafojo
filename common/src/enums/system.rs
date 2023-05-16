use serde::{Deserialize, Serialize};

/**
 * 资源类型枚举 todo 等会写成宏 先试一试
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnumResourceType {
    Common,
    Menu,
}

impl TryFrom<String> for EnumResourceType {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value {
            value if value == "1" => Ok(EnumResourceType::Common),
            value if value == "2" => Ok(EnumResourceType::Menu),
            _ => Err(()),
        }
    }
}

impl From<EnumResourceType> for String {
    fn from(val: EnumResourceType) -> String {
        match val {
            EnumResourceType::Common => "1".to_owned(),
            EnumResourceType::Menu => "2".to_owned(),
        }
    }
}
