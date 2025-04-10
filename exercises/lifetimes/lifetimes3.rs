// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.


// 问题：我们用 'a 指定生命周期，那这个 'a 到底是多长？它也不确定啊？那岂不是还是可能传进来一个无效引用？
// 你只是说“这些引用必须活得一样久”，但是否真的“活那么久”，要看调用者传什么！编译器会在每次调用时严格检查引用的实际生命周期。
// 不同的生命周期：fn pick_first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
// 相当于将使用生命周期注释的部分，不论是 struct 还是 fn 都以它单独的方式来进行考虑，考虑需要的生命周期，有一种解耦的感觉。
// 即：函数内部怎么写是我的事，使用者能不能满足约束是编译器的事
// ❌ 只要结构体包含引用类型字段，你就必须手动写生命周期注释：
// 如果 fn 使用了引用参数，但是返回值不是引用, 不用写，因为生命周期注释就是为了确定返回值的生命周期
// 然后有一些可以省略的部分，就是
// 1. 只有一个引用
// 2. 多个引用但是其中有&self
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
