use yew::prelude::*;
use yew::Renderer;

// #[function_component(Counter)]
// fn counter() -> Html {
//     let count = use_state(|| 0);

//     let increase = {
//         let count = count.clone();
//         Callback::from(move |_| count.set(*count + 1))
//     };

//     let decrease = {
//         let count = count.clone();
//         Callback::from(move |_| count.set(*count - 1))
//     };

//     html! {
//         <div>
//             <h1>{ "Yew Counter App" }</h1>
//             <p>{ format!("Count: {}", *count) }</p>
//             <button onclick={increase}>{ "Increment" }</button>
//             <button onclick={decrease}>{ "Decrement" }</button>
//         </div>
//     }
// }

#[function_component(App)]
fn app() -> Html {
    // html! { <Counter /> }
    let name=use_state(|| String::from(""));

    let on_input={
        let name=name.clone();
        Callback::from(move |e:InputEvent|{
            let input:web_sys::HtmlInputElement=e.target_dyn_into().unwrap();
            name.set(input.value());
        })
    };

    html!{
        <div>
            <h1>{"Hello, Yew!"}</h1>
            <input type="text" placeholder="Enter your name" oninput={on_input} />
            <p>{format!("Hello, {}!",*name)}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
