use yew::prelude::*;
use crate::User;

#[function_component(Profile)]
pub fn profile() -> Html {
    let user = use_context::<User>().expect("No user context found.");
    let username = user.username.borrow().clone();

    html! {
        <div class="flex flex-col items-center justify-center w-full h-full bg-gray-100 p-8">
            <img
                src="https://img.freepik.com/free-vector/blue-circle-with-white-user_78370-4707.jpg?semt=ais_hybrid&w=740"
                alt="Profile Image"
                class="w-32 h-32 rounded-full shadow-md mb-4 object-cover"
            />
            <h2 class="text-xl sm:text-2xl font-semibold text-gray-800">{ format!("Hello, {}!", username) }</h2>
        </div>
    }
}
