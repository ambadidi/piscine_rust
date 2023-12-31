pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        let twin = self.ref_list.clone();
        let mut result: Vec<Rc<String>> = Vec::new();
        for ind in 0..twin.len(){
            if !Rc::ptr_eq(&twin[ind], &element){
                result.push(twin[ind].clone());
            }
        }

        self.ref_list = result;
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}