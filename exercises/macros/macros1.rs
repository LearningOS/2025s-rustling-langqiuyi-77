// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


// macro_rules! 定义宏
macro_rules! my_macro {
    // 模式匹配和展开
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 调用宏
    my_macro!();
}
