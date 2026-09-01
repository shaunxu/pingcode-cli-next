//! 不遵循 module/resource/operation 三级模式的自由命令。
//!
//! 每个自由命令一个文件，直接在 [`crate::commands::run`] 的顶层 match 中分发，
//! 不需要向三级命令那样声明资源/操作枚举。

pub mod state;
pub mod whoami;
