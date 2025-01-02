use leptos::{
    component, create_effect, create_signal, use_context, view, IntoView, ReadSignal, SignalGet,
};
use leptos_router::{RouterContext, A};

const INPUT_STYLE: &str = "border-b-0 border-[#7734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";
const INPUT_STYLE_SELECTED: &str =
    "border-b-2 border-[#9734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";

#[component]
pub fn Header() -> impl IntoView {
    let (current_path, set_current_path) = create_signal(String::new());

    create_effect(move |_| {
        let router_context = use_context::<RouterContext>();
        match router_context {
            Some(route_context) => {
                let path = route_context.pathname().get();
                set_current_path(path);
            }
            None => set_current_path(String::from("/")),
        }
    });

    view! {
        <div class="flex fixed top-0 items-center px-20 pt-8 mx-auto w-full h-12 align-center">
            <nav class="flex flex-row w-full h-12 max-w-[52rem]">
                <div class=move || get_style_from_url(&current_path, "/")>
                    <A href="/">"Dashboard"</A>
                </div>
                <div class=move || get_style_from_url(&current_path, "/team")>
                    <A href="/team">"Team"</A>
                </div>
            </nav>
        </div>
    }
}

fn get_style_from_url<'a, 'b>(url: &'a ReadSignal<String>, match_url: &'a str) -> &'b str {
    if url() == match_url {
        INPUT_STYLE_SELECTED
    } else {
        INPUT_STYLE
    }
}
