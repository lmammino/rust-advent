use std::fmt::Display;

#[derive(Debug, Default)]
pub struct CharList {
    nodes_pool: Vec<CharListNode>,
}

impl<T: AsRef<str>> From<T> for CharList {
    fn from(s: T) -> CharList {
        let mut char_list: CharList = Default::default();

        for c in s.as_ref().chars() {
            char_list.push(c);
        }

        char_list
    }
}

impl Display for CharList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.chars() {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl CharList {
    pub fn get(&self, index: usize) -> Option<&CharListNode> {
        self.nodes_pool.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut CharListNode> {
        self.nodes_pool.get_mut(index)
    }

    pub fn next(&self, index: usize) -> Option<&CharListNode> {
        let current = self.nodes_pool.get(index);

        if let Some(node) = current {
            if let Some(next_index) = node.next {
                return self.get(next_index);
            }
        }

        None
    }

    pub fn push(&mut self, value: char) {
        let new_node_id = self.insert_value(value);
        if new_node_id > 0 {
            let prev = new_node_id - 1;
            self.nodes_pool.get_mut(prev).unwrap().next = Some(new_node_id);
        }
    }

    pub fn insert_value(&mut self, value: char) -> usize {
        let list_node = CharListNode { value, next: None };
        let new_index = self.nodes_pool.len();
        self.nodes_pool.push(list_node);
        new_index
    }

    pub fn insert_value_with_next(&mut self, value: char, next: usize) -> usize {
        let list_node = CharListNode {
            value,
            next: Some(next),
        };
        let new_index = self.nodes_pool.len();
        self.nodes_pool.push(list_node);
        new_index
    }

    pub fn update_next(&mut self, index: usize, new_next: usize) {
        if let Some(node) = self.get_mut(index) {
            node.next = Some(new_next);
        }
    }

    pub fn chars(&self) -> CharListIter<'_> {
        CharListIter {
            list: self,
            current_index: Some(0),
        }
    }
}

#[derive(Debug, Default)]
pub struct CharListNode {
    pub value: char,
    pub next: Option<usize>,
}

#[derive(Debug)]
pub struct CharListIter<'a> {
    list: &'a CharList,
    current_index: Option<usize>,
}

impl<'a> Iterator for CharListIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_index) = self.current_index {
            if let Some(node) = self.list.get(current_index) {
                self.current_index = node.next;
                return Some(node.value);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::CharList;

    #[test]
    fn test_from_str() {
        let list: CharList = "NCNBCHB".into();
        assert_eq!(list.to_string(), "NCNBCHB");
    }
}
