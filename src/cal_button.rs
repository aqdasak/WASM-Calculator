use yew::prelude::*;

// #[derive(Clone, PartialEq)]
// pub struct CalButton {
//     pub number: i64,
// }

// #[derive(Properties, PartialEq)]
// pub struct ClBtnProps {
//     pub number: i64,
//     pub on_click: Callback<i64>,
// }

// #[function_component(ClBtn)]
// pub fn cal_button(ClBtnProps { number, on_click }: &ClBtnProps) -> Html {
//     let number = number.clone();
//     let on_click = on_click.clone();

//     // let cal_button = CalButton { number: *number };
//     html! {
//         // <button onclick={Callback::from(move |_| on_click.emit(cal_button.clone()))}>{number}</button>
//         <button onclick={Callback::from(move |_| on_click.emit(number))}>{number}</button>
//     }
// }

#[derive(Properties, PartialEq)]
pub struct ClBtnProps {
    pub value: String,
    pub on_click: Callback<String>,
}

#[function_component(ClBtn)]
pub fn cal_button(ClBtnProps { value, on_click }: &ClBtnProps) -> Html {
    let value = value.clone();
    let on_click = on_click.clone();

    // let cal_button = CalButton { number: *number };
    let value_clone = value.clone();
    html! {
        // <button onclick={Callback::from(move |_| on_click.emit(cal_button.clone()))}>{number}</button>
        <button onclick={Callback::from(move |_| on_click.emit(value.clone()))}>{value_clone}</button>
    }
}
