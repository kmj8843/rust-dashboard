use std::rc::Rc;

use leptos::{
    component, create_resource, create_signal, view, CollectView, IntoView, Show, SignalGet,
    Suspense,
};

use crate::app::{
    components::{AddPersonModal, Header, PersonRow, Toast, ToastMessage},
    server_functions::get_persons,
};

#[component]
pub fn TeamPage() -> impl IntoView {
    const ADD_BUTTON_STYLE: &str = "bg-[#7734e7] px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

    let (if_show_modal, set_if_show_modal) = create_signal(false);
    let (if_show_toast, set_if_show_toast) = create_signal(false);
    let (toast_message, set_toast_message) = create_signal(ToastMessage::new());

    let get_persons_info = create_resource(|| (), |_| async move { get_persons().await });

    let on_click = move |_| set_if_show_modal(!if_show_modal());

    view! {
        <div class="justify-center items-center mx-auto w-full max-w-[64rem] align-middle">
            <Header />

            <Toast toast_message if_appear=if_show_toast set_if_appear=set_if_show_toast />

            <div class="mt-20">
                <div class="flex flex-col justify-center items-center mx-auto w-full text-white z-25">
                    <Show when=move || { if_show_modal() }>
                        <AddPersonModal
                            set_if_show_modal
                            set_if_show_added=set_if_show_toast
                            set_toast_message
                            person_resource=get_persons_info
                        />
                    </Show>
                    <div class="flex flex-row w-full max-w-[52rem]">
                        <div class="pr-4 mt-4 text-xl">"Members"</div>
                        <hr class="pt-4 pr-4 pl-4 mt-8 mr-4 w-full max-w-[48rem]" />
                        <button on:click=on_click class=ADD_BUTTON_STYLE>
                            "Add"
                        </button>
                    </div>

                    <Suspense fallback=move || {
                        view! { <p>"Loading..."</p> }
                    }>
                        <div class="flex flex-col w-full max-w-[52rem] mt-6">
                            {move || {
                                get_persons_info
                                    .get()
                                    .map(|data| {
                                        match data {
                                            Ok(persons_data) => {
                                                persons_data
                                                    .iter()
                                                    .map(|each_person| {
                                                        view! {
                                                            <PersonRow
                                                                person=Rc::new(each_person.clone())
                                                                person_resource=get_persons_info
                                                                set_if_show_toast
                                                                set_toast_message
                                                            />
                                                        }
                                                    })
                                                    .collect_view()
                                            }
                                            Err(_) => view! { <div></div> }.into_view(),
                                        }
                                    })
                            }}
                        </div>
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
