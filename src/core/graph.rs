pub type NodeId = usize;

#[derive(Debug)]
pub struct Node<T>(pub NodeId, pub T);

pub struct NodeArena<T> (Vec<Node<T>>, Option<usize>);

impl<T> NodeArena<T> {
    pub fn new() -> NodeArena<T> {
        NodeArena::<T>(Default::default(), Default::default())
    }

    pub fn alloc(&mut self, value: T) -> NodeId {
        let id = self.0.len();
        let node = Node(id, value);
        self.0.push(node);
        self.1 = Some(id);
        id
    }

    pub fn get(&self, id: NodeId) -> &Node<T> {
        &self.0[id]
    }

    pub fn get_mut(&mut self, id: NodeId) -> &mut Node<T> {
        &mut self.0[id]
    }

    pub fn entry(&self) -> Option<NodeId> {
        self.1
    }

    pub fn set_entry(&mut self, id: NodeId) {
        self.1 = Some(id);
    }

    pub fn max_id(&self) -> Option<NodeId> {
        let id = self.0.len();
        if id == 0 { None } else { Some(id - 1) }
    }
}

#[test]
fn make() { let arena = NodeArena::<u8>(vec![]); }

#[test]
fn alloc() {
    let mut arena = NodeArena::<u8>(vec![]);
    assert_eq!(arena.alloc(46), 0);
}

#[test]
fn get() {
    let mut arena = NodeArena::<u8>(vec![]);
    let node_id = arena.alloc(46);

}

#[test]
fn get_mut() {
    let mut arena = NodeArena::<u8>(vec![]);
    let node_id = arena.alloc(46);
    {
        let mut node = arena.get_mut(node_id);
        node.value = 87;
    }
    assert_eq!(arena.get(node_id).value, 87);
}
