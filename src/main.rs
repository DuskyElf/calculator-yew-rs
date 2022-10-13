use std::rc::Rc;
use yew::prelude::*;
use calculator_yew_rs::*;

#[function_component]
fn App() -> Html {
    use Operation::*;
    let state = use_state(|| State{
        operation: None,
        previous_operand: None,
        current_operand: Num::Int(0),
    });

    let handle_action = Rc::from({
        let state = state.clone();

        Callback::from(move |action: Action|
            match action {
                _ => todo!(),
            }
        )
    });

    html! {
        <div class="calculator-grid">
            <div class="output">
                <div class="previous-operand"></div>
                <div class="current-operand"></div>
            </div>
            <button class="span-two">{"AC"}</button>
            <button>{"DEL"}</button>
            <OpButton operation={Div} handler={Rc::clone(&handle_action)}/>
            <DigitButton digit=1 handler={Rc::clone(&handle_action)}/>
            <DigitButton digit=2 handler={Rc::clone(&handle_action)}/>
            <DigitButton digit=3 handler={Rc::clone(&handle_action)}/>
            <OpButton operation={Mul} handler={Rc::clone(&handle_action)}/>
            <DigitButton digit=4 handler={Rc::clone(&handle_action)}/>
            <DigitButton digit=5 handler={Rc::clone(&handle_action)}/>
            <DigitButton digit=6 handler={Rc::clone(&handle_action)}/>
            <OpButton operation={Add} handler={Rc::clone(&handle_action)}/>
            <DigitButton digit=7 handler={Rc::clone(&handle_action)}/>
            <DigitButton digit=8 handler={Rc::clone(&handle_action)}/>
            <DigitButton digit=9 handler={Rc::clone(&handle_action)}/>
            <OpButton operation={Sub} handler={Rc::clone(&handle_action)}/>
            <button>{"."}</button>
            <DigitButton digit=0 handler={Rc::clone(&handle_action)}/>
            <button class="span-two">{"="}</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
