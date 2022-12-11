/*
    Appellation: backend <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{BaseApplication, BaseObject, Stateful, Versionable};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Backend<S: Default + Stateful> {
    pub count: usize,
    pub name: String,
    pub namespace: String,
    pub state: Arc<S>,
    pub symbol: String,
    pub version: String,
}

impl<S: Default + Stateful> Backend<S> {
    pub fn new(name: String, namespace: String, symbol: String, version: Option<String>) -> Self {
        let (count, state) = (0, Arc::new(Default::default()));
        let version = version.unwrap_or_else(|| String::from("v0.1.0"));
        Self {
            count,
            name,
            namespace,
            state,
            symbol,
            version,
        }
    }
}

impl<S: Default + Stateful> BaseObject for Backend<S> {
    fn count(&self) -> usize {
        self.count
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn symbol(&self) -> String {
        self.symbol.clone()
    }
}

impl<S: Default + Stateful> Stateful for Backend<S> {}

impl<S: Default + Stateful> Versionable for Backend<S> {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn update(&mut self) -> Result<(), Box<Self::Error>> {
        Ok(())
    }

    fn version(&self) -> String {
        self.version.clone()
    }
}

impl<S: Default + Stateful> BaseApplication for Backend<S> {
    fn namespace(&self) -> String {
        self.namespace.clone()
    }
}

impl<S: Default + Stateful> Default for Backend<S> {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            Default::default(),
            None,
        )
    }
}
