use yew::prelude::*;
mod math_parser;
use math_parser::eval;

mod cal_button;
use cal_button::ClBtn;

#[function_component(App)]
fn app() -> Html {
    let calculate = use_state(|| "".to_string());

    let on_click = {
        let calculate = calculate.clone();
        Callback::from(move |button_value: String| {
            calculate.set(String::from((*calculate).clone()) + &button_value)
        })
    };

    let calculate2 = calculate.clone();
    let clear = {
        Callback::from(move |_| calculate2.set((calculate2[..calculate2.len() - 1]).to_string()))
    };

    let calculate2 = calculate.clone();
    let all_clear = { Callback::from(move |_| calculate2.set("".to_string())) };

    html! {
        <>
            <center>
                <h1>{ "Calculator" }</h1>
                <p id="expression">{&*calculate}</p>
                <p>{eval(&*calculate)}</p>
                <div class="container">
                    <div id="calpad-clear">
                        <button onclick={clear}>{"⌫"}</button>
                        <button onclick={all_clear}>{"AC"}</button>
                    </div>
                    <div id="calpad">
                        <ClBtn value="1" on_click={on_click.clone()}/>
                        <ClBtn value="2" on_click={on_click.clone()}/>
                        <ClBtn value="3" on_click={on_click.clone()}/>
                        <ClBtn value="+" on_click={on_click.clone()}/>

                        <ClBtn value="4" on_click={on_click.clone()}/>
                        <ClBtn value="5" on_click={on_click.clone()}/>
                        <ClBtn value="6" on_click={on_click.clone()}/>
                        <ClBtn value="-" on_click={on_click.clone()}/>

                        <ClBtn value="7" on_click={on_click.clone()}/>
                        <ClBtn value="8" on_click={on_click.clone()}/>
                        <ClBtn value="9" on_click={on_click.clone()}/>
                        <ClBtn value="*" on_click={on_click.clone()}/>
                        <ClBtn value="00" on_click={on_click.clone()}/>
                        <ClBtn value="0" on_click={on_click.clone()}/>
                        <ClBtn value="." on_click={on_click.clone()}/>
                        <ClBtn value="/" on_click={on_click.clone()}/>

                    </div>
                </div>
            </center>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
