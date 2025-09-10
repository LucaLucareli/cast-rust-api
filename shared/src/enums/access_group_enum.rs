use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
#[serde(into = "i32")]
#[allow(non_camel_case_types)]
pub enum AccessGroupEnum {
    VIEWER = 1,
    PREMIUM = 2,
    ADMIN = 3,
    SUPER_ADMIN = 4,
}

impl From<i32> for AccessGroupEnum {
    fn from(value: i32) -> Self {
        match value {
            1 => AccessGroupEnum::VIEWER,
            2 => AccessGroupEnum::PREMIUM,
            3 => AccessGroupEnum::ADMIN,
            4 => AccessGroupEnum::SUPER_ADMIN,
            _ => AccessGroupEnum::VIEWER,
        }
    }
}

impl From<AccessGroupEnum> for i32 {
    fn from(group: AccessGroupEnum) -> Self {
        group as i32
    }
}
