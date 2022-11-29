use yew::prelude::*;
use reqwest;
#[function_component(App)]
pub fn app() -> Html {
    let client = reqwest::blocking::Client::new();
    html! {
        <main>
            <h1>{ "Hello World!" }</h1>
            <h1> {"Yew testing!"} </h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
