use leptos::{
    component, create_signal, event_target_value, spawn_local, view, IntoView, Resource,
    ServerFnError, Show, WriteSignal,
};
use validator::Validate;

use crate::app::{
    models::{AddPersonRequest, Person},
    server_functions::persons::add_person,
};

use super::{Toast, ToastMessage, ToastMessageType};

#[component]
pub fn AddPersonModal(
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_added: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
    person_resource: Resource<(), Result<Vec<Person>, ServerFnError>>,
) -> impl IntoView {
    const INPUT_STYLE: &str = "w-full h-12 bg-[#333333] pr-4 pl-6 py-4 text-white mt-6 outline-none focus:outline:none focus:pl-7 transition-all duration-1000 ease-in-out";
    const CANCLE_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";
    const ADD_BUTTON_STYLE:&str = "mt-10 bg-[#7734e7] px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";
    const NO_ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7 px-6 pt-5 h-[29rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";
    const ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7 px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";

    let (person_name, set_person_name) = create_signal(String::new());
    let (person_title, set_person_title) = create_signal(String::new());
    let (person_level, set_person_level) = create_signal(String::new());
    let (compensation, set_compensation) = create_signal(String::new());

    let (error_message, set_error_message) = create_signal(String::new());
    let (if_error, set_if_error) = create_signal(false);

    let on_close = move |_| {
        set_if_show_modal(false);
    };

    let on_click = move |_| {
        let add_person_request = AddPersonRequest::new(
            person_name(),
            person_title(),
            person_level(),
            compensation().parse::<i32>().expect("Numbers only"),
        );

        let is_valid = add_person_request.validate();

        match is_valid {
            Ok(_) => spawn_local(async move {
                let add_result = add_person(add_person_request).await;

                match add_result {
                    Ok(_added_person) => {
                        person_resource.refetch();
                        set_if_show_modal(false);

                        set_toast_message(ToastMessage::create(ToastMessageType::NewMemberAdded));
                        set_if_show_added(true);
                    }
                    Err(e) => println!("Error adding {:?}", e),
                }
            }),
            Err(_) => {
                set_if_error(true);
                set_error_message(String::from("All fields are required"));
            }
        }
    };

    view! {
        <div class="flex z-50 flex-col items-center mx-auto w-full h-full align-middle">
            <div class=move || { if if_error() { ERROR_STYLE } else { NO_ERROR_STYLE } }>
                <Show when=move || { if_error() }>
                    <p class="py-3 px-5 w-full h-12 text-white bg-red-500 rounded transition-all ease-in-out duration-750">
                        {error_message()}
                    </p>
                </Show>
                <p class="pt-5 text-white">"Add New Employee"</p>
                <input
                    type="text"
                    placeholder="Name"
                    class=INPUT_STYLE
                    value=person_name
                    on:input=move |event| {
                        set_person_name(event_target_value(&event));
                    }
                />
                <input
                    type="text"
                    placeholder="Title"
                    class=INPUT_STYLE
                    value=person_title
                    on:input=move |event| {
                        set_person_title(event_target_value(&event));
                    }
                />
                <input
                    type="text"
                    placeholder="Level"
                    class=INPUT_STYLE
                    value=person_level
                    on:input=move |event| {
                        set_person_level(event_target_value(&event));
                    }
                />
                <input
                    type="text"
                    placeholder="Compensation"
                    class=INPUT_STYLE
                    value=compensation
                    on:input=move |event| {
                        set_compensation(event_target_value(&event));
                    }
                />
                <div class="flex flex-row justify-end w-full items-right">
                    <button on:click=on_close class=CANCLE_BUTTON_STYLE>
                        "Cancel"
                    </button>
                    <button on:click=on_click class=ADD_BUTTON_STYLE>
                        "Add"
                    </button>
                </div>
            </div>
        </div>
    }
}
