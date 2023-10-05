use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

use crate::runtime::RuntimeObject;

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
#[serde(try_from = "Vec<ContainerElement>")]
pub struct Container {
    pub content: Vec<RuntimeObject>,
    pub named_subelements: HashMap<String, RuntimeObject>,
    pub name: Option<String>,
    pub visits_should_be_counted: bool,
    pub turn_index_should_be_counted: bool,
    pub count_at_start_only: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ContainerElement {
    RuntimeObject(RuntimeObject),
    SpecialFinal(Option<ContainerData>),
}

#[derive(Debug, Deserialize, Default)]
struct ContainerData {
    #[serde(rename = "#n")]
    name: Option<String>,
    #[serde(rename = "#f", default)]
    flags: u8,
    #[serde(flatten)]
    named_subelements: HashMap<String, RuntimeObject>,
}

#[derive(Debug, Error)]
enum ContainerError {
    #[error("Failed to deserialize Container, expected object or null, found {0}")]
    UnexpectedRuntimeObject(RuntimeObject),
    #[error("Failed to deserialize Container, no elements")]
    NoElements,
    #[error("Failed to deserialize Container element as RuntimeObject, found null")]
    UnexpectedNull,
    #[error("Failed to deserialize Container element as RuntimeObject, found map")]
    UnexpectedMapObject(ContainerData),
}

impl Container {
    pub fn new() -> Container {
        Container::default()
    }

    pub fn count_flags(&self) -> u8 {
        let mut count_flags: u8 = 0;

        if self.visits_should_be_counted {
            count_flags &= 0x1;
        }

        if self.turn_index_should_be_counted {
            count_flags &= 0x2;
        }

        if self.count_at_start_only {
            count_flags &= 0x4;
        }

        if count_flags == 0x4 {
            0
        } else {
            count_flags
        }
    }

    pub fn set_count_flags(&mut self, count_flags: u8) {
        self.visits_should_be_counted = count_flags & 0x1 > 0;
        self.turn_index_should_be_counted = count_flags & 0x2 > 0;
        self.count_at_start_only = count_flags & 0x4 > 0;
    }

    pub fn add_child(&mut self, obj: RuntimeObject) {
        self.content.push(obj);
    }

    pub fn prepend(&mut self, mut objects: Vec<RuntimeObject>) {
        objects.append(&mut self.content);
        self.content = objects;
    }

    /*pub fn get_content_from_path_component(&self, component: &PathComponent)-> Option<&RuntimeObject> {
        match component {
            &PathComponent::Index(ref index_component) => {
                let index = index_component.index();

                if index < self.content.len()  {
                    self.content.get(index)
                } else {
                    None
                }
            },
            &PathComponent::Named(ref named_component) => {
                if named_component.is_parent() {
                    // self.parent()
                    None
                }
                else {
                    // TODO
                    None
                }
            }
        }
    }*/

    pub fn search_by_name(&self, name: &str) -> Option<&RuntimeObject> {
        for runtime_object in &self.content {
            if let Some(other_name) = runtime_object.name() {
                if name == other_name {
                    return Some(runtime_object);
                }
            }
        }

        None
    }
}

impl TryFrom<Vec<ContainerElement>> for Container {
    type Error = ContainerError;

    fn try_from(mut elements: Vec<ContainerElement>) -> Result<Container, ContainerError> {
        // take last element of Container
        let data = match elements.pop() {
            Some(ContainerElement::SpecialFinal(Some(data))) => data,
            Some(ContainerElement::SpecialFinal(None)) => ContainerData::default(),
            Some(ContainerElement::RuntimeObject(object)) => {
                return Err(ContainerError::UnexpectedRuntimeObject(object))
            }
            None => return Err(ContainerError::NoElements),
        };
        // map other elements to RuntimeObject
        let content = elements
            .into_iter()
            .map(|item| match item {
                ContainerElement::RuntimeObject(element) => Ok(element),
                ContainerElement::SpecialFinal(None) => Err(ContainerError::UnexpectedNull),
                ContainerElement::SpecialFinal(Some(found)) => {
                    Err(ContainerError::UnexpectedMapObject(found))
                }
            })
            .collect::<Result<_, _>>()?;
        let visits_should_be_counted = data.flags & 0x1 > 0;
        let turn_index_should_be_counted = data.flags & 0x2 > 0;
        let count_at_start_only = data.flags & 0x4 > 0;
        Ok(Container {
            content,
            named_subelements: data.named_subelements,
            name: data.name,
            visits_should_be_counted,
            turn_index_should_be_counted,
            count_at_start_only,
        })
    }
}
