use std::collections::HashMap;

use crate::{models::TypeIdent, pipeline::interpreter::state::NodeDependency};

use super::{Error, Node, State};

impl State<'_> {
    /// Creates a ordered list of type identifiers from the [NodeCache](super::NodeCache).
    ///
    /// This method constructs a comprehensive list of [`TypeIdent`] values that represent the type
    /// nodes within the current parsing state. It internally handles the complex logic of:
    /// - Traversing the parsed XSD structure
    /// - Identifying and collecting all type definitions
    /// - Organizing them into a structured node list
    /// - Resolving type references and dependencies
    ///
    /// The returned list is ordered in a way that respects the dependencies between types,
    /// ensuring that any type is listed after all of its strong dependencies have been listed.
    /// This is crucial for the subsequent code generation phase, where types need to be
    /// defined in an order that allows for correct referencing.
    pub(super) fn create_node_list(&mut self) -> Result<Vec<TypeIdent>, Error> {
        MakeNodeList::new(self).exec()
    }
}

struct MakeNodeList<'state, 'schema> {
    state: &'state State<'schema>,
    nodes: Vec<TypeIdent>,
    visited: HashMap<TypeIdent, MakeNodeListState>,
}

enum MakeNodeListState {
    Visited,
    Done,
}

impl<'state, 'schema> MakeNodeList<'state, 'schema> {
    fn new(state: &'state State<'schema>) -> Self {
        Self {
            state,
            nodes: Vec::new(),
            visited: HashMap::new(),
        }
    }

    fn exec(mut self) -> Result<Vec<TypeIdent>, Error> {
        for ident in self.state.node_cache.keys() {
            if !matches!(self.visited.get(ident), Some(MakeNodeListState::Done)) {
                self.visit_node(false, ident)?;
            }
        }

        Ok(self.nodes)
    }

    fn visit_node(&mut self, lazy: bool, ident: &TypeIdent) -> Result<(), Error> {
        match self.visited.get(ident) {
            Some(MakeNodeListState::Visited) => {
                return Err(Error::CircularDependency(ident.clone()));
            }
            Some(MakeNodeListState::Done) => return Ok(()),
            None => (),
        }

        if !lazy {
            self.visited
                .insert(ident.clone(), MakeNodeListState::Visited);
        }

        if let Some(entry) = self.state.node_cache.get(ident) {
            for dep in entry.dependencies.values() {
                let ret = match dep {
                    NodeDependency::Weak(_) => continue,
                    NodeDependency::Strong(ident) => self.visit_node(false, ident),
                    NodeDependency::Lazy(ident) => self.visit_node(true, ident),
                };

                match ret {
                    Ok(()) => (),
                    Err(Error::CircularDependency(_)) => {
                        return Err(Error::CircularDependency(ident.clone()));
                    }
                    Err(e) => return Err(e),
                }
            }

            if !lazy
                && matches!(
                    entry.node,
                    Node::Element(_)
                        | Node::Attribute(_)
                        | Node::SimpleType(_)
                        | Node::ComplexType(_)
                )
            {
                self.nodes.push(ident.clone());
            }
        }

        if !lazy {
            self.visited.insert(ident.clone(), MakeNodeListState::Done);
        }

        Ok(())
    }
}
