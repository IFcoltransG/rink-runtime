use std::rc::Rc;

use serde::Deserialize;

use crate::{
    path::{Fragment, Path},
    runtime::container::Container,
    runtime::RuntimeObject,
};

#[derive(Debug, Deserialize)]
pub struct RuntimeGraph {
    #[serde(rename = "inkVersion")]
    pub ink_version: u32,
    #[serde(rename = "root")]
    pub root_container: Rc<Container>,
}

impl RuntimeGraph {
    pub fn resolve_path(&self, path: &Path) -> Option<&RuntimeObject> {
        let mut current_container = &self.root_container;
        let mut runtime_object: Option<&RuntimeObject> = None;

        let mut it = path.fragments.iter();
        while let Some(fragment) = it.next() {
            match fragment {
                &Fragment::Index(index) => match current_container.content.get(index) {
                    Some(child) => {
                        if let &RuntimeObject::Container(ref container) = child {
                            current_container = container;
                        }

                        runtime_object = Some(child);
                    }
                    _ => return None,
                },
                &Fragment::Name(ref name) => match current_container.search_by_name(name) {
                    Some(child) => {
                        if let &RuntimeObject::Container(ref container) = child {
                            current_container = container;
                        }

                        runtime_object = Some(child);
                    }
                    _ => return None,
                },
            }
        }

        runtime_object
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_path_by_name_test() {
        let path = Path::from_str("a.b.c");

        let mut root_container = Container::new();

        let mut child_level_1 = Container::new();
        child_level_1.name = Some("a".to_owned());

        let mut child_level_2 = Container::new();
        child_level_2.name = Some("b".to_owned());

        let mut child_level_3 = Container::new();
        child_level_3.name = Some("c".to_owned());

        child_level_2.add_child(RuntimeObject::Container(Rc::new(child_level_3)));
        child_level_1.add_child(RuntimeObject::Container(Rc::new(child_level_2)));
        root_container.add_child(RuntimeObject::Container(Rc::new(child_level_1)));

        let graph = RuntimeGraph {
            ink_version: 17,
            root_container: root_container.into(),
        };

        match graph.resolve_path(&path.unwrap()) {
            Some(&RuntimeObject::Container(ref container)) => {
                assert_eq!(container.name.as_ref().unwrap(), "c")
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn resolve_path_by_index_test() {
        use crate::runtime::divert::{Divert, TargetType};

        let path = Path::from_str("a.b.1");

        let mut root_container = Container::new();

        let mut child_level_1 = Container::new();
        child_level_1.name = Some("a".to_owned());

        let mut child_level_2 = Container::new();
        child_level_2.name = Some("b".to_owned());

        let mut child_level_3_1 = Container::new();
        child_level_3_1.name = Some("c".to_owned());

        let child_level_3_2 = Divert::new(TargetType::VarName("mytarget".to_owned()));

        child_level_2.add_child(RuntimeObject::Container(Rc::new(child_level_3_1)));
        child_level_2.add_child(RuntimeObject::Divert(child_level_3_2));
        child_level_1.add_child(RuntimeObject::Container(Rc::new(child_level_2)));
        root_container.add_child(RuntimeObject::Container(Rc::new(child_level_1)));

        let graph = RuntimeGraph {
            ink_version: 17,
            root_container: root_container.into(),
        };

        match graph.resolve_path(&path.unwrap()) {
            Some(&RuntimeObject::Divert(ref divert)) => match &divert.target {
                &TargetType::VarName(ref name) => assert_eq!(name, "mytarget"),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }
}
