// iterators5.rs
//
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
//
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

// map 是对 HashMap 的 引用
// value 是传值进去的，是 Progress 类型
// map.values() 返回的是 &Progress 的迭代器
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // map 是引用，得到value，key是因为他会自动解运用
    // 然后重点是他返回的，实际上是引用，相当于又加了一个运用，他自动处理
    // 这是rust的哲学，遍历时不复制值，默认只借用它们
    // 另一个就是，Rust默认是move而不是copy
    // into_iter() 这种取就是真的取到了，而且所有权都移动了，真正消耗了map
    // 默认行为是“借用不复制、解引用不手动”。
    map.values().filter(|val| *val == &value).count()
}

// &[HashMap<String, Progress>] 这个传的是一个HashMap类型切片的引用
fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // // 这里迭代器得到的是 HashMap<String, Progress> 的引用 m
    // collection.iter().for_each(|m| {
    //     m.iter().for_each(|(key, val)| {
    //         if val == &value {
    //             count += 1;
    //         }
    //     });
    // }); 不可，因为for_each是是只读的环境，但是在内部却更改了外部变量count 报错
    collection.iter()
        // 这里 collection 是引用，所以 m 是 HashMap 的引用，然后 m.values() 
        // 这里 .filter 是对 m.values() 的结果调用，而 m.values() 本身是一个 迭代器，它产生的是 &Progress。
        // 然后你在 .filter(|val| ...) 里，Rust 会自动对值借用一次传给闭包里的 val，所以：
        // val: &&Progress   // yes！引用的引用！
        .map(|m| m.values().filter(|val| *val == &value).count())
        .sum()
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
    collection.iter()
        // 这里 collection 是引用，所以 m 是 HashMap 的引用，然后 m.values() 
        // 这里 .filter 是对 m.values() 的结果调用，而 m.values() 本身是一个 迭代器，它产生的是 &Progress。
        // 然后你在 .filter(|val| ...) 里，Rust 会自动对值借用一次传给闭包里的 val，所以：
        // val: &&Progress   // yes！引用的引用！
        .map(|m| m.values().filter(|val| *val == &value).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(1, count_iterator(&map, Progress::Some));
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(2, count_iterator(&map, Progress::None));
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state)
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(1, count_collection_iterator(&collection, Progress::Some));
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(4, count_collection_iterator(&collection, Progress::None));
    }

    #[test]
    fn count_collection_equals_for() {
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        let collection = get_vec_map();

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state)
            );
        }
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}
