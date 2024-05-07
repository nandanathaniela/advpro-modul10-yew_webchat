use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-gradient-to-b from-pink-100 to-pink-300 flex w-screen h-screen items-center justify-center">
            <div class="bg-white shadow-lg rounded-lg p-8">
                <form class="flex">
                    <input {oninput} class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-pink-50 focus:outline-none focus:border-pink-300 focus:ring-1 focus:ring-pink-300" placeholder="Username"/>
                    <Link<Route> to={Route::Chat}> <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-pink-600 text-white font-bold p-4 uppercase border-pink-600 border-t border-b border-r hover:bg-pink-700 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:ring-opacity-50 transition-colors" >{"Go Chatting!"}</button></Link<Route>>
                </form>
            </div>
        </div>
    }
}