use std::collections::HashMap;

pub trait HashMapStr<T> {
    fn get_from_str(&self, member: &str) -> Option<T>;
}

impl<T> HashMapStr<T> for HashMap<String, T>
where
    T: Clone,
{
    fn get_from_str(&self, member: &str) -> Option<T> {
        match self.get(&member.to_string()) {
            Some(m) => Some(m.clone()),
            None => None,
        }
    }
}
