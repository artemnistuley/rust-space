use std::cmp::Ordering;

struct Key {
    id: u32,
    metadata: Option<String>,
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Eq, PartialEq)]
struct Citation {
    author: String,
    year: u32,
}

impl PartialOrd for Citation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.author.partial_cmp(&other.author) {
            Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
            author_ord => author_ord,
        }
    }
}

fn main() {
    let k1 = Key { id: 1, metadata: Some(String::from("some metadata")) };
    let k2 = Key { id: 2, metadata: Some(String::from("some metadata")) };
    println!("{}", k1 == k2);

    let c1 = Citation { author: String::from("a"), year: 1992 };
    let c2 = Citation { author: String::from("b"), year: 1993 };
    println!("{}", c2 > c1);
}
