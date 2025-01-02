use std::rc::Rc;

use leptos::{component, create_signal, view, IntoView};

use crate::app::models::Person;

const ROW_STYLE: &str = "bg-[#283653] rounded px-10 py-5 mb-4 flex flex-row text-left items-left transition-all duration-1000 ease-in-out";
const SHOW_ICON_STYLE: &str = "bg-transparent border-2 border-white px-2.5 rounded-full text-white transition-all duration-500 ease-in-out text-xs mr-3 w-7 h-7 hover:w-8 hover:h-8 hover:mb-1";

#[component]
pub fn PersonRow(person: Rc<Person>) -> impl IntoView {
    let (if_show_edit_modal, set_if_show_edit_modal) = create_signal(false);

    let on_show_edit = move |_| {
        set_if_show_edit_modal(true);
    };

    view! {
        <div class=ROW_STYLE>
            <div class="flex flex-col w-full max-w-[45rem]">
                <p class="font-bold">{&person.name}</p>
                <p class="text-sm text-stone-400">{&person.title}</p>
            </div>

            <div class="flex flex-row">
                <button class="" on:click=on_show_edit>
                    <img src="assets/edit.png" class="w-[35px] hover:w-[38px] transition-all duration-500" />
                </button>
            </div>
        </div>
    }
}
