use std::rc::Rc;

use core::*;

pub type NodeId = usize;

#[derive(Clone)]
pub struct Node<T>(pub NodeId, pub T);

impl fmt::Debug for Node<Epiq> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {:?})", self.0, self.1)
    }
}

impl fmt::Debug for Node<Rc<Epiq>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {:?})", self.0, self.1)
    }
}

pub struct SymbolTable {
    table: Vec<Vec<(String, Option<NodeId>)>>,
    current_index: usize,
}

impl SymbolTable {
    pub fn new(_initial_table: Vec<(String, Option<NodeId>)>) -> SymbolTable {
        SymbolTable {
            table: vec![vec![]],
            current_index: Default::default(),
        }
    }

    pub fn define(&mut self, name: &str, value: NodeId) {
        // println!("symbol_table: {:?}", "define");
        if {
            if let Some(&(_, ref _r)) = self.table[self.current_index].iter().find(|&&(ref n, _)| n == name) {
                // すでに含まれていたら上書きしたいが、方法がわからないので何もせずにおく
                // *r = Some(value);
                false
            } else {
                true
            }
        } {
            self.table[self.current_index].push( (name.to_string(), Some(value)) );
            // log(format!("symbol_table: {:?}", self.table));
        }
    }

    pub fn resolve(&self, name: &str) -> Option<Option<NodeId>> {
        self.resolve_internal(name, self.current_index)
    }

    fn resolve_internal(&self, name: &str, frame: usize) -> Option<Option<NodeId>> {
        if let Some(&( _, Some(r) )) = self.table[frame].iter().find(|&&(ref n, _)| n == name) {
            // println!("resolve 見つかりました {:?} {:?}", name, self.table);
            Some(Some(r))
        } else {
            if frame == 0 {
                // グローバル環境まで来たら終了
                // println!("resolve 見つかりませんでした {:?} {:?}", name, self.table);
                None
            } else {
                // なかったら一つ上のframeを探す
                // println!("resolve 見つからないので親を探します {:?}", name);
                self.resolve_internal(name, frame - 1)
            }
        }
    }

    pub fn extend(&mut self) {
        let new_frame = vec![];
        self.table.push(new_frame);
        self.current_index = self.table.len() - 1;

    }

    pub fn pop(&mut self) {
        let _ = self.table.pop();
        self.current_index = self.table.len() - 1;
    }
}

pub struct NodeArena<T> (pub Vec<Node<T>>, Option<usize>, SymbolTable);

impl<T> NodeArena<T> {
    pub fn new() -> NodeArena<T> {
        NodeArena::<T>(Default::default(), Default::default(), SymbolTable::new(vec![]))
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

    pub fn define(&mut self, name: &str, value: NodeId) {
        self.2.define(name, value)
    }

    pub fn resolve(&self, name: &str) -> Option<Option<&Node<T>>> {
        match self.2.resolve(name) {
            Some(Some(n)) => {
                Some(Some(&self.0[n]))
            },
            _ => {
                // どちらかがNoneだが今は考えない
                // とりあえずNone
                None
            }
        }

    }

    pub fn extend(&mut self) {
        self.2.extend()
    }

    pub fn pop(&mut self) {
        self.2.pop()
    }
}

#[test]
fn make() { let _arena = NodeArena::<u8>::new(); }

#[test]
fn alloc() {
    let mut arena = NodeArena::<u8>::new();
    assert_eq!(arena.alloc(46), 0);
}

#[test]
fn get() {
    let mut arena = NodeArena::<u8>::new();
    let _node_id = arena.alloc(46);

}

#[test]
fn get_mut() {
    let mut arena = NodeArena::<u8>::new();
    let node_id = arena.alloc(46);
    {
        let node = arena.get_mut(node_id);
        node.1 = 87;
    }
    assert_eq!(arena.get(node_id).1, 87);
}
