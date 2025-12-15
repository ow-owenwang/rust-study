/*
Newtype 是指将一个已有类型包裹在一个结构体中，从而创建一个全新的类型：


用途：
类型安全（区分相同底层类型）
可为新类型实现 trait（即使原类型不能）
 */

struct UserId(u32);

fn get_user_name(id: UserId) {
    // 明确区分 UserId 与 u32
}

// 通过 Deref 或 From 实现原类型的行为：
impl From<u32> for UserId {
    fn from(value: u32) -> Self {
        UserId(value)
    }
}
