use std::rc::Rc;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn print(&self) -> char {
        match self {
            Operation::Add => '+',
            Operation::Sub => '-',
            Operation::Mul => '*',
            Operation::Div => '/',
        }
    }
}

#[derive(Copy, Clone)]
pub enum Num {
    Int(i64),
    Float(f64),
}

impl Num {
    pub fn print(&self) -> String {
        match self {
            Self::Int(num) => format!("{num}"),
            Self::Float(num) => format!("{num}"),
        }
    }

    pub fn insert_digit(self, digit: u8) -> Self {
        match self {
            Self::Int(num) =>{
                if num < i64::MAX / 10 {
                    Self::Int(num * 10 + digit as i64)
                }
                else {
                    self
                }
            }
            Self::Float(_) => todo!(),
        }
    }
}

pub enum Action {
    Clear,
    Evaluate,
    AddDecimal,
    DeleteDigit,
    AddDigit(u8),
    AddOperation(Operation),
}

#[derive(Properties, PartialEq, Clone)]
pub struct DigitButtonProp {
    pub digit: u8,
    pub handler: Rc<Callback<Action>>,
}

#[function_component]
pub fn DigitButton(props: &DigitButtonProp) -> Html {
    let onclick = {
        let props = props.clone();
        Callback::from(move |_|
            props.handler.emit(Action::AddDigit(props.digit))
        )
    };

    html! {
        <button {onclick}> { props.digit } </button>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct OpButtonProp {
    pub operation: Operation,
    pub handler: Rc<Callback<Action>>,
}

#[function_component]
pub fn OpButton(props: &OpButtonProp) -> Html {
    let onclick = {
        let props = props.clone();
        Callback::from(move |_|
            props.handler.emit(Action::AddOperation(props.operation.clone()))
        )
    };

    html! {
        <button {onclick}> { props.operation.print() } </button>
    }
}
