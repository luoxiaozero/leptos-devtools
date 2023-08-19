#[derive(Debug, Clone)]
pub(crate) struct Component {
    name: String,
    target: String,
    props: Option<String>,
}

impl Component {
    pub fn new(name: String, target: String) -> Self {
        Self {
            name,
            target,
            props: None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn target(&self) -> &String {
        &self.target
    }

    pub fn props(&self) -> &Option<String> {
        &self.props
    }

    pub fn set_props(&mut self, props: Option<String>) {
        self.props = props;
    }
}
