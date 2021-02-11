use std::cmp::Ordering;

#[derive(Debug)]
struct BinarySearchTreeST<K, V>
where
    K: Ord,
{
    key: Option<K>,
    value: Option<V>,
    size: i32,
    right: Option<Box<BinarySearchTreeST<K, V>>>,
    left: Option<Box<BinarySearchTreeST<K, V>>>,
}

impl<K, V> BinarySearchTreeST<K, V>
where
    K: Ord,
{
    fn new() -> Self {
        BinarySearchTreeST {
            key: Option::None,
            value: Option::None,
            size: 1,
            right: Option::None,
            left: Option::None,
        }
    }

    fn size(&self) -> i32 {
        self.size
    }

    fn size_node(node: &Option<Box<BinarySearchTreeST<K, V>>>) -> i32 {
        match node {
            Option::Some(x) => x.size(),
            Option::None => 0,
        }
    }

    fn get(&self, key: K) -> Option<&V> {
        match &self.key {
            Option::Some(k) => match key.cmp(&k) {
                Ordering::Less => match &self.left {
                    Option::Some(node) => node.get(key),
                    Option::None => Option::None,
                },
                Ordering::Greater => match &self.right {
                    Option::Some(node) => node.get(key),
                    Option::None => Option::None,
                },
                Ordering::Equal => {
                    if let Option::Some(v) = &self.value {
                        Option::Some(v)
                    } else {
                        Option::None
                    }
                }
            },
            Option::None => Option::None,
        }
    }

    fn put(&mut self, key: K, value: V) {
        match &self.key {
            Option::Some(k) => match key.cmp(k) {
                Ordering::Less => match &mut self.left {
                    Option::Some(node) => node.put(key, value),
                    Option::None => {
                        let mut node = Self::new();
                        node.put(key, value);
                        self.left = Option::Some(Box::new(node));
                    }
                },
                Ordering::Greater => match &mut self.right {
                    Option::Some(node) => node.put(key, value),
                    Option::None => {
                        let mut node = Self::new();
                        node.put(key, value);
                        self.right = Option::Some(Box::new(node));
                    }
                },
                Ordering::Equal => {
                    self.value = Option::Some(value);
                }
            },
            Option::None => {
                self.key = Option::Some(key);
                self.value = Option::Some(value);
            }
        }
        self.size = Self::size_node(&self.left) + Self::size_node(&self.right) + 1;
    }

    fn min(&self) -> Option<&V> {
        match &self.left {
            Option::Some(node) => node.min(),
            Option::None => match &self.value {
                Option::Some(v) => Option::Some(v),
                Option::None => Option::None,
            },
        }
    }

    fn max(&self) -> Option<&V> {
        match &self.right {
            Option::Some(node) => node.max(),
            Option::None => match &self.value {
                Option::Some(v) => Option::Some(v),
                Option::None => Option::None,
            },
        }
    }

    fn floor(&self, key: K) -> Option<&V> {
        match &self.key {
            Option::Some(k) => match key.cmp(k) {
                Ordering::Less => match &self.left {
                    Option::Some(node) => node.floor(key),
                    Option::None => Option::None,
                },
                Ordering::Equal => match &self.value {
                    Option::Some(v) => Option::Some(v),
                    Option::None => Option::None,
                },
                Ordering::Greater => match &self.right {
                    Option::Some(node) => match node.floor(key) {
                        Option::Some(v) => Option::Some(v),
                        Option::None => match &self.value {
                            Option::Some(v) => Option::Some(v),
                            Option::None => Option::None,
                        },
                    },
                    Option::None => match &self.value {
                        Option::Some(v) => Option::Some(v),
                        Option::None => Option::None,
                    },
                },
            },
            Option::None => Option::None,
        }
    }

    fn ceiling(&self, key: K) -> Option<&V> {
        match &self.key {
            Option::Some(k) => match key.cmp(k) {
                Ordering::Greater => match &self.right {
                    Option::Some(node) => node.ceiling(key),
                    Option::None => Option::None,
                },
                Ordering::Equal => match &self.value {
                    Option::Some(v) => Option::Some(v),
                    Option::None => Option::None,
                },
                Ordering::Less => match &self.left {
                    Option::Some(node) => match node.ceiling(key) {
                        Option::Some(v) => Option::Some(v),
                        Option::None => match &self.value {
                            Option::Some(v) => Option::Some(v),
                            Option::None => Option::None,
                        },
                    },
                    Option::None => match &self.value {
                        Option::Some(v) => Option::Some(v),
                        Option::None => Option::None,
                    },
                },
            },
            Option::None => Option::None,
        }
    }
}

fn main() {
    let mut st = BinarySearchTreeST::<i32, String>::new();
    st.put(15, String::from("This"));
    st.put(11, String::from("Is"));
    st.put(17, String::from("A"));
    st.put(12, String::from("Test"));
    st.put(16, String::from("That"));
    st.put(19, String::from("Not"));

    println!("{:#?}", st);

    if let Option::Some(val) = st.get(12) {
        println!("{}", &val);
    } else {
        println!("Key not found");
    }
    println!("{}", st.size());
    if let Option::Some(val) = st.min() {
        println!("Value associated with min key is {:?}", &val);
    }
    if let Option::Some(val) = st.max() {
        println!("Value associated with max key is {:?}", &val);
    }
    if let Option::Some(val) = st.floor(16) {
        println!("floor = {:?}", &val);
    } else {
        println!("floor value could not be found!");
    }
    if let Option::Some(val) = st.ceiling(10) {
        println!("ceiling = {:?}", &val);
    } else {
        println!("ceiling value could not be found!");
    }
}
