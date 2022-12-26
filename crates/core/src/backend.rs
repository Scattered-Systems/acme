/*
    Appellation: backend <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{BaseApplication, BaseObject, Versionable};
use scsys::prelude::{State, StatePack};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Backend<S: StatePack> {
    pub count: usize,
    pub name: String,
    pub namespace: String,
    pub state: Arc<State<S>>,
    pub symbol: String,
    pub version: String,
}

impl<S: StatePack> Backend<S> {
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

impl<S: StatePack> BaseObject for Backend<S> {
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

impl<S: StatePack> Versionable for Backend<S> {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn update(&mut self) -> Result<(), Box<Self::Error>> {
        Ok(())
    }

    fn version(&self) -> String {
        self.version.clone()
    }
}

impl<S: StatePack> BaseApplication for Backend<S> {
    fn namespace(&self) -> String {
        self.namespace.clone()
    }
}

impl<S: StatePack> Default for Backend<S> {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            Default::default(),
            None,
        )
    }
}
