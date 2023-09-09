#[derive(Debug, Clone)]
pub(crate) struct Component {
    // Data
    name: String,
    props: Option<String>,
    // Info
    location: Option<String>,
    // Devtools
    target: String,
}

impl Component {
    pub fn new(name: String, location: Option<String>, target: String) -> Self {
        Self {
            name,
            props: None,
            location,
            target,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn props(&self) -> &Option<String> {
        &self.props
    }

    pub fn set_props(&mut self, props: Option<String>) {
        self.props = props;
    }

    pub fn location(&self) -> &Option<String> {
        &self.location
    }

    pub fn target(&self) -> &String {
        &self.target
    }
}
