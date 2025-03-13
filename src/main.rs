use yew::prelude::*;
use yew::Renderer;

#[function_component(Counter)]
fn counter() -> Html {
    let count = use_state(|| 0);

    let increase = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };

    let decrease = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count - 1))
    };

    html! {
        <div>
            <h1>{ "Yew Counter App" }</h1>
            <p>{ format!("Count: {}", *count) }</p>
            <button onclick={increase}>{ "Increment" }</button>
            <button onclick={decrease}>{ "Decrement" }</button>
        </div>
    }
}x

#[function_component(App)]
fn app() -> Html {
    html! { <Counter /> }
}

fn main() {
    Renderer::<App>::new().render();
}
