// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars(); // 把字符串转成字符迭代器
    // 如果你想获取字符序列，你得明确说：我想按“字符”迭代
    // 而不是 iter，因为只iter根本不知道按什么迭代
    match c.next() {
        None => String::new(),
        Some(first) => {
            
            let capitalized = first.to_uppercase().to_string();
            // to_uppercase() 返回一个迭代器，因为有些字符的「大写形式」不止一个字符（比如德语的 ß → SS）。
            // 所以我们再 .to_string() 转换成字符串（也可以用 .collect()）
            let rest: String = c.collect(); // 把剩下的字符收集起来
            capitalized + &rest
        }
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
// 如果你用 Vec<&str>，那别人只能传 Vec。如果你用 &[&str]，别人可以传 Vec、数组、甚至别的切片。
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // &str 是一个字符串切片（string slice）——比如 "hello" 这样的只读借用字符串。
    // &[T] 是对一个切片的引用，可以理解为数组的只读借用。
    // 对一个由字符串切片组成的切片的引用
    words.iter()    // 遍历每个单词（得到 &&str
    // Rust 的引用是：一个 只读/只写受限的指针，带有生命周期约束，并且经过编译器的借用检查器
    // .iter() 是标准库写的，它返回的是元素的引用
    .map(|w| capitalize_first(w)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut mystring = String::new();
    // words.iter.map(|w| capitalize_first(w)) 这里啥都没做
    // 不能直接 push，因为 map() 只是“惰性迭代器”，它不会自动执行任何操作,你得显式地用 .for_each() 或者 .collect::<Vec<_>>()，才能触发实际动作。
    for word in words {
        mystring.push_str(&capitalize_first(word));
    }
    // 闭包使用，但是并不好，因为iter()返回的是不可变迭代器，但是却借用了可变变量，，，
    // words.iter().for_each(|w| {
    //     mystring.push_str(&capitalize_first(w));
    // });
    mystring
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
