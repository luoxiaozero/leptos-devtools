#[derive(Debug, Clone)]
pub(crate) struct Component {
    name: String,
    target: String,
}

impl Component {
    pub fn new(name: String, target: String) -> Self {
        Self {
            name,
            target
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn target(&self) -> &String {
        &self.target
    }
}