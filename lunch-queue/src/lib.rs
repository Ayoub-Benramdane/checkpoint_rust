#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }
    pub fn add(&mut self, name: String, discount: i32) {
        let person = Some(Box::new(Person { discount, name, next: self.node.take() }));
        self.node = person;
    }
    pub fn invert_queue(&mut self) {
        let mut cur = self.node.take();
        let mut prev = None;
        while let Some(mut res) = cur {
            let next = res.next.take();
            res.next = prev.take();
            prev = Some(res);
            cur = next;
        }
        self.node = prev;
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut  cur = &mut self.node;
        if cur.as_ref().unwrap().next.is_none() {
            let node = cur.take().unwrap();
            return Some((node.name, node.discount));
        }
        while let Some(ref mut res) = cur {
            if res.next.as_ref().unwrap().next.is_none(){
            let node = res.next.take().unwrap();
            return Some((node.name, node.discount));
            }
            cur = &mut res.next;
        }
        None
    }
    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut cur = &self.node;
        while let Some(res) = cur {
            cur = &res.next;
            if res.name == name {
                return Some((res.name.clone(), res.discount));
            }
        }
        None
    }
}
