use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct BSTNode<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    key: K,
    value: V,
    size: i32,
    right: Option<Box<BSTNode<K, V>>>,
    left: Option<Box<BSTNode<K, V>>>,
}

struct ST<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    root: Option<Box<BSTNode<K, V>>>,
}

impl<K, V> ST<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn size(&self) -> i32 {
        Self::size_node(&self.root)
    }

    fn size_node(node: &Option<Box<BSTNode<K, V>>>) -> i32 {
        match node {
            Option::Some(x) => x.size,
            Option::None => 0,
        }
    }

    fn get(&self, key: K) -> Option<&V> {
        Self::get_node(&self.root, key)
    }

    fn get_node<'a>(node: &'a Option<Box<BSTNode<K, V>>>, key: K) -> Option<&'a V> {
        match node {
            Option::None => Option::None,
            Option::Some(x) => match key.cmp(&x.key) {
                Ordering::Less => return Self::get_node(&x.left, key),
                Ordering::Greater => return Self::get_node(&x.right, key),
                Ordering::Equal => return Option::Some(&x.value),
            },
        }
    }

    fn put(&mut self, key: K, value: V) {
        self.root = Self::put_node(&mut self.root, key, value);
    }

    fn put_node(
        node: &mut Option<Box<BSTNode<K, V>>>,
        key: K,
        value: V,
    ) -> Option<Box<BSTNode<K, V>>> {
        if let Option::Some(x) = node {
            match key.cmp(&x.key) {
                Ordering::Less => x.left = Self::put_node(&mut x.left, key, value),
                Ordering::Greater => x.right = Self::put_node(&mut x.right, key, value),
                Ordering::Equal => x.value = value,
            }
            x.size = Self::size_node(&x.left) + Self::size_node(&x.right) + 1;
            return Option::Some(x.clone());
        } else {
            return Option::Some(Box::new(BSTNode {
                key,
                value,
                size: 1,
                right: Option::None,
                left: Option::None,
            }));
        }
    }
}

fn main() {
    let mut st = ST::<i32, String> { root: Option::None };
    st.put(13, String::from("This"));
    st.put(12, String::from("Hello"));
    st.put(11, String::from("That"));

    if let Option::Some(val) = st.get(11) {
        println!("{}", &val);
    } else {
        println!("Key not found");
    }
}
