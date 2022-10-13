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

pub enum Num {
    Int(i64),
    Float(f64),
}

pub enum Action {
    Clear,
    Evaluate,
    AddDecimal,
    DeleteDigit,
    AddDigit(u8),
    AddOperation(Operation),
}

pub struct State {
    pub current_operand: Num,
    pub operation: Option<Operation>,
    pub previous_operand: Option<Num>,
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
