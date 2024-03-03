use leptos::html::Input;
use leptos::*;

use web_sys::SubmitEvent;

#[component]
pub fn Introduction(
    game_started: ReadSignal<bool>,
    set_game_started: WriteSignal<bool>,
    set_username: WriteSignal<String>,
    class_name: &'static str,
) -> impl IntoView {
    let (enter_name_line, set_enter_name_line) = create_signal("Enter your name".to_string());
    let input_element: NodeRef<Input> = create_node_ref();

    let check_empty_name = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();

        if value.is_empty() {
            set_enter_name_line("Please enter your name!".to_string());
        } else {
            set_username(value);
            set_game_started(true);
        }
    };

    let render_introduction = move || {
        if game_started() {
            view! { "" }.into_view()
        } else {
            view! { class = class_name,
            <div id="introduction" {game_started}>
                <p id="nameRequire">{enter_name_line}</p>
                <form on:submit=check_empty_name>
                <input type="text" id="playerName"
                    node_ref=input_element/>
                <br/>
                <button id="playerNameButton">
                        Enter
                </button>
                </form>
                </div>
            }
            .into_view()
        }
    };

    view! {
        {render_introduction}
    }
}
