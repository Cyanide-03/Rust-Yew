use yew::prelude::*;
use yew::Renderer;

// ! For an app which increments and decrements a counter
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


// ! For an app which takes user input and displays a greeting
// #[function_component(App)]
// fn app() -> Html {
    // html! { <Counter /> }
// }

// #[function_component(App)]
// fn app() -> Html {
//     let name=use_state(|| String::from(""));

//     let on_input={
//         let name=name.clone();
//         Callback::from(move |e:InputEvent|{
//             let input:web_sys::HtmlInputElement=e.target_dyn_into().unwrap();
//             name.set(input.value());
//         })
//     };

//     html!{
//         <div>
//             <h1>{"Hello, Yew!"}</h1>
//             <input type="text" placeholder="Enter your name" oninput={on_input} />
//             <p>{format!("Hello, {}!",*name)}</p>
//         </div>
//     }
// }

// ! For an app which let the user click a button and display a message
#[function_component(App)]
fn app() -> Html {
    let message=use_state(|| String::new());

    let on_click={
        let message=message.clone();
        Callback::from(move |_|{
            message.set("You clicked the button!".to_string());
        })
    };

    html!{
        <div>
            <h1>{"Hello, Yew!"}</h1>
            <button onclick={on_click}>{ "Click me!" }</button>
            <p>{(*message).clone()}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
