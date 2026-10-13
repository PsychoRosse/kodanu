use crate::{Color, DrawList, Node, NodeAllocator, NodeId, Quad, Rect, Style};

pub struct UiContext {
    allocator: NodeAllocator,
    nodes: Vec<Option<Node>>,
}

impl Default for UiContext {
    fn default() -> Self {
        Self {
            allocator: NodeAllocator::default(),
            nodes: vec![Some(Node::new(Style::default()))],
        }
    }
}

impl UiContext {
    pub const fn root(&self) -> NodeId {
        NodeId::ROOT
    }

    pub fn create_node(&mut self, style: Style) -> NodeId {
        let id = self.allocator.allocate();
        let index = id.index() as usize;

        if index == self.nodes.len() {
            self.nodes.push(Some(Node::new(style)));
        } else {
            self.nodes[index] = Some(Node::new(style));
        }

        id
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        if !self.allocator.contains(id) {
            return None;
        }

        self.nodes.get(id.index() as usize)?.as_ref()
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        if !self.allocator.contains(id) {
            return None;
        }

        self.nodes.get_mut(id.index() as usize)?.as_mut()
    }

    pub fn add_child(&mut self, parent: NodeId, child: NodeId) -> bool {
        if parent == child || self.get(parent).is_none() || self.get(child).is_none() {
            return false;
        }

        let mut ancestor = Some(parent);

        while let Some(id) = ancestor {
            if id == child {
                return false;
            }

            ancestor = self.get(id).and_then(Node::parent);
        }

        let old_parent = self.get(child).and_then(Node::parent);

        if let Some(old_parent) = old_parent
            && let Some(node) = self.get_mut(old_parent)
        {
            node.remove_child(child);
        }

        self.get_mut(parent)
            .expect("parent was validated")
            .add_child(child);

        self.get_mut(child)
            .expect("child was validated")
            .set_parent(Some(parent));

        true
    }

    pub fn remove_node(&mut self, id: NodeId) -> bool {
        if id == NodeId::ROOT || self.get(id).is_none() {
            return false;
        }

        let mut stack = vec![id];
        let mut subtree = Vec::new();

        while let Some(current) = stack.pop() {
            let Some(node) = self.get(current) else {
                continue;
            };

            stack.extend_from_slice(node.children());
            subtree.push(current);
        }

        if let Some(parent) = self.get(id).and_then(Node::parent)
            && let Some(parent_node) = self.get_mut(parent)
        {
            parent_node.remove_child(id);
        }

        for current in subtree.into_iter().rev() {
            let index = current.index() as usize;

            self.nodes[index] = None;
            self.allocator.deallocate(current);
        }

        true
    }

    pub fn set_rect(&mut self, id: NodeId, rect: Rect) -> bool {
        let Some(node) = self.get_mut(id) else {
            return false;
        };

        node.set_rect(rect);
        true
    }

    pub fn build_draw_list(&self, draw_list: &mut DrawList) {
        draw_list.clear();

        let root = self.root();

        let Some(node) = self.get(root) else {
            return;
        };

        for &child in node.children() {
            self.build_draw_list_node(child, draw_list);
        }
    }

    fn build_draw_list_node(&self, id: NodeId, draw_list: &mut DrawList) {
        let Some(node) = self.get(id) else {
            return;
        };

        draw_list.push(Quad {
            rect: node.rect(),
            color: Color::rgba(0.15, 0.15, 0.18, 1.0),
            node: id,
        });

        for &child in node.children() {
            self.build_draw_list_node(child, draw_list);
        }
    }

    pub fn len(&self) -> usize {
        self.allocator.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
