use std::rc::Rc;
use yew::prelude::*;
use calculator_yew_rs::*;

#[function_component]
fn App() -> Html {
    use Operation::*;

    type PrState = UseStateHandle<Option<Num>>;
    type OpState = UseStateHandle<Option<Operation>>;
    let operation: OpState = use_state(|| None);
    let current_operand = use_state(|| Num::Int(0));
    let previous_operand: PrState = use_state(|| None);

    let handle_action = Rc::from({
        let operation = operation.clone();
        let current_operand = current_operand.clone();
        let previous_operand = previous_operand.clone();

        Callback::from(move |action: Action|
            match action {
                Action::AddDigit(digit) => {
                    current_operand.set(current_operand.insert_digit(digit));
                }

                Action::Clear => {
                    operation.set(None);
                    previous_operand.set(None);
                    current_operand.set(Num::Int(0));
                }
                _ => todo!(),
            }
        )
    });

    html! {
        <div class="calculator-grid">
            <div class="output">
                <div class="previous-operand"></div>
                <div class="current-operand">{ current_operand.print() }</div>
            </div>

            <button class="span-two" onclick={
                let handle_action = handle_action.clone();
                Callback::from(move |_|
                    Rc::clone(&handle_action)
                        .emit(Action::Clear)
                )
            }>{"AC"}</button>

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
