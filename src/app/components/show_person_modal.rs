use std::rc::Rc;

use leptos::{component, spawn_local, view, IntoView, Resource, ServerFnError, WriteSignal};

use crate::app::{
    components::{Toast, ToastMessageType},
    models::{DeletePersonRequest, Person},
    server_functions::persons::delete_person,
};

use super::ToastMessage;

const INFO_STYLE: &str = "w-full h-12 pr-4 py-4 mt-6 flex flex-col outline-none focus:outline-none focus:pl-7 transition-all duration-1000 ease-in-out";
const INFO_TITLE_STYLE: &str = "text-stone-400 text-xs";
const INFO_VALUE_STYLE: &str = "text-white";
const CLOSE_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";
const DELETE_BUTTON_STYLE: &str = "mt-10 bg-red-800 px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-red-600";
const MODAL_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7] px-6 py-5 h-[28rem] w-full max-w-[36rem] z-50 -mt-2 fixed top-20 z-50";

#[component]
pub fn ShowPersonModal(
    person: Rc<Person>,
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_deleted: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
    person_resource: Resource<(), Result<Vec<Person>, ServerFnError>>,
) -> impl IntoView {
    let this_person = person.clone();

    let on_close = move |_| {
        set_if_show_modal(false);
    };
    let on_click_delete = move |_| {
        let to_delete_uuid = this_person.uuid.to_string();
        let delete_person_request = DeletePersonRequest::new(to_delete_uuid);
        spawn_local(async move {
            let delete_result = delete_person(delete_person_request).await;

            match delete_result {
                Ok(_deleted_person) => {
                    person_resource.refetch();
                    set_toast_message(ToastMessage::create(ToastMessageType::MemberDeleted));
                    set_if_show_deleted(true);
                    set_if_show_modal(false);
                }
                Err(err) => println!("Error deleting {:?}", err),
            };
        });
    };
    view! {
        <div class="flex flex-col w-full h-full z-49 bg-[#222222]/[.06]">
            <div class="flex flex-col w-full h-full z-50 mx-auto items-center align-middle">
                <div class=MODAL_STYLE>
                    <p class="text-white pt-5 text-4xl mb-2 mt-2">{&person.name}</p>
                    <div class=INFO_STYLE>
                        <div class=INFO_TITLE_STYLE>"Title"</div>
                        <div class=INFO_VALUE_STYLE>{&person.title}</div>
                    </div>
                    <div class=INFO_STYLE>
                        <div class=INFO_TITLE_STYLE>"Level"</div>
                        <div class=INFO_VALUE_STYLE>{&person.level}</div>
                    </div>
                    <div class=INFO_STYLE>
                        <div class=INFO_TITLE_STYLE>"Compensation"</div>
                        <div class=INFO_VALUE_STYLE>{format!("{:?}", &person.compensation)}</div>
                    </div>
                    <div class="flex flex-row w-full items-right justify-end mt-3">
                        <button on:click=on_close class=CLOSE_BUTTON_STYLE>
                            "Close"
                        </button>
                        <button on:click=on_click_delete class=DELETE_BUTTON_STYLE>
                            "Delete"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
