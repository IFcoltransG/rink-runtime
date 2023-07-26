pub mod choice_point;
pub mod container;
pub mod control_command;
pub mod divert;
pub mod glue;
pub mod native_function_call;
pub mod tag;
pub mod value;
pub mod variable;

use std::fmt;
use std::rc::Rc;

use runtime::choice_point::ChoicePoint;
use runtime::container::Container;
use runtime::control_command::ControlCommand;
use runtime::divert::Divert;
use runtime::glue::Glue;
use runtime::native_function_call::NativeFunctionCall;
use runtime::tag::Tag;
use runtime::value::Value;
use runtime::variable::{ReadCount, VariableAssignment, VariableReference};

pub enum RuntimeObject {
    Choice(ChoicePoint),
    Container(Rc<Container>),
    ControlCommand(ControlCommand),
    Divert(Divert),
    Glue(Glue),
    NativeFunctionCall(NativeFunctionCall),
    Tag(Tag),
    Value(Value),
    VariableAssignment(VariableAssignment),
    VariableReference(VariableReference),
    ReadCount(ReadCount),
    Void,
    Null,
}

impl fmt::Display for RuntimeObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &RuntimeObject::ControlCommand(ref control_command) => {
                write!(f, "{}", control_command.to_string())
            }
            _ => write!(f, "TODO"),
        }
    }
}

impl RuntimeObject {
    pub fn is_container(&self) -> bool {
        match self {
            &RuntimeObject::Container(_) => true,
            _ => false,
        }
    }

    pub fn as_container(&self) -> Option<&Rc<Container>> {
        match self {
            &RuntimeObject::Container(ref container) => Some(container),
            _ => None,
        }
    }

    pub fn as_value(&self) -> Option<&Value> {
        match self {
            &RuntimeObject::Value(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn name(&self) -> Option<&str> {
        match *self {
            RuntimeObject::Container(ref container) => container.name(),
            // TODO
            _ => None,
        }
    }
}
