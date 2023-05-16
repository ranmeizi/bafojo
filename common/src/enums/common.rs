use serde::{Deserialize, Serialize};

/**
 * 资源类型枚举 todo 等会写成宏 先试一试
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnumEnabled {
    Enabled,
    Disabled,
}

impl From<i8> for EnumEnabled {
    fn from(val: i8) -> Self {
        match val {
            1 => EnumEnabled::Enabled,
            0 => EnumEnabled::Disabled,
            _ => EnumEnabled::Enabled
        }
    }
}

impl From<EnumEnabled> for i8 {
    fn from(val: EnumEnabled) -> i8 {
        match val {
            EnumEnabled::Enabled => 1,
            EnumEnabled::Disabled => 0,
        }
    }
}

impl From<EnumEnabled> for bool {
    fn from(val: EnumEnabled) -> bool {
        match val {
            EnumEnabled::Enabled => true,
            EnumEnabled::Disabled => false,
        }
    }
}
