type NodeId = usize;

pub struct Node<T> {
    id: NodeId,
    pub value : T,
}

pub struct NodeArena<T> (Vec<Node<T>>);

impl<T> NodeArena<T> {
    pub fn new() -> NodeArena<T> {
        NodeArena::<T>(Default::default())
    }

    pub fn alloc(&mut self, value: T) -> NodeId {
        let id = self.0.len();
        let node = Node{id, value};
        self.0.push(node);
        id
    }

    pub fn get(&self, id: NodeId) -> &Node<T> {
        &self.0[id]
    }

    pub fn get_mut(&mut self, id: NodeId) -> &mut Node<T> {
        &mut self.0[id]
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
