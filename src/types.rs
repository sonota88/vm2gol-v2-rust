pub type NodeId = usize;

static mut NODE_ID_MAX: usize = 0;
static mut NODES: Vec<Node> = vec![];

// --------------------------------

pub type Chars = Vec<char>;

pub fn chars_from(s: &str) -> Chars {
    s.chars().collect()
}

// --------------------------------

#[derive(Debug)]
pub enum NodeVal {
    Int(i32),
    Str(String),
    List(List),
}

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub val: NodeVal,
}

impl Node {
    pub fn new(
        val: NodeVal,
    ) -> NodeId {
        let newid: NodeId;
        unsafe {
            newid = NODE_ID_MAX;
        }

        let node = Node {
            id: newid,
            val: val
        };

        unsafe {
            NODE_ID_MAX += 1;
            NODES.push(node);
        }

        newid
    }

    pub fn new_int(n: i32) -> NodeId {
        Node::new(
            NodeVal::Int(n)
        )
    }

    pub fn new_str(s: String) -> NodeId {
        Node::new(
            NodeVal::Str(s)
        )
    }

    pub fn new_list(list: List) -> NodeId {
        Node::new(
            NodeVal::List(list)
        )
    }
}

// --------------------------------

#[derive(Debug)]
pub struct List {
    node_ids: Vec<NodeId>,
}

impl List {
    pub fn new() -> List {
        List {
            node_ids: vec![]
        }
    }

    pub fn size(&self) -> usize {
        self.node_ids.len()
    }

    pub fn get(&self, i: usize) -> &Node {
        if self.node_ids.len() <= i {
            panic!("index out of bound")
        }

        let id = self.node_ids[i];

        let node: &Node;
        unsafe {
            node = &NODES[id];
        }

        node
    }

    pub fn get_list(&self, i: usize) -> &List {
        let node: &Node = self.get(i);
        match &node.val {
            NodeVal::List(list) => list,
            _ => panic!("invalid kind")
        }
    }

    pub fn get_str(&self, i: usize) -> &str {
        let node: &Node = self.get(i);
        match &node.val {
            NodeVal::Str(s) => s,
            _ => panic!("invalid kind")
        }
    }

    pub fn rest(&self, n: usize) -> List {
        let mut new_one = List::new();

        let mut i = n;
        while i < self.size() {
            new_one.add_node(self.get(i).id);
            i += 1;
        }

        return new_one;
    }

    pub fn add_node(&mut self, node_id: NodeId) {
        self.node_ids.push(node_id);
    }

    pub fn add_int(&mut self, n: i32) {
        let node_id = Node::new_int(n);
        self.add_node(node_id);
    }

    pub fn add_str(&mut self, s: &str) {
        let node_id = Node::new_str(s.to_string());
        self.add_node(node_id);
    }

    pub fn add_list(&mut self, list: List) {
        let node_id = Node::new_list(list);
        self.add_node(node_id);
    }

    pub fn add_all(&mut self, list: &List) {
        let mut i = 0;
        while i < list.size() {
            let node = list.get(i);
            self.add_node(node.id);
            i += 1;
        }
    }

}

// --------------------------------

#[derive(Debug)]
pub struct Token {
    pub kind: String,
    pub value: String,
}

impl Token {
    pub fn new(kind: &str, value: &str) -> Token {
        Token {
            kind: kind.to_string(),
            value: value.to_string()
        }
    }
}
