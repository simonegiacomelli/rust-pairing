pub enum Option1<T> {
    None,
    Some(T),
}

impl<T> Option1<T> {
    fn unwrap_or(self, other: T) -> T {
        match self {
            Option1::Some(t) => t,
            Option1::None => other
        }
    }

    // fn unwrap_or_2(&self, other: T) -> T {
    //     match self {
    //         Option1::Some(t) => *t,
    //         //                  ^^ move occurs because `*t` has type `T`, which does not implement
    //         Option1::None => other
    //     }
    // }
}

fn main() {}