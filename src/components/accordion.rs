use leptos::prelude::*;

#[component]
pub fn Accordion(#[prop(into)] title: &'static str, children: Children) -> impl IntoView {
    let is_open = RwSignal::new(false);

    view! {
        <div class="border-b border-gray-200">
            <button
                class="flex justify-between items-center py-3 px-4 w-full font-medium text-left text-gray-700 transition-colors cursor-pointer hover:bg-gray-50"
                on:click=move |_| is_open.update(|v| *v = !*v)
            >
                <span>{title}</span>
                <span
                    class="text-gray-500 transition-transform"
                    class=("rotate-180", move || is_open.get())
                >
                    "▼"
                </span>
            </button>
            <div style=move || {
                if is_open.get() {
                    "max-height: 1000px; opacity: 1; overflow: hidden; transition: max-height 0.5s ease, opacity 0.35s ease; margin-top: 0.5rem; padding-left: 1rem;"
                } else {
                    "max-height: 0px; opacity: 0; overflow: hidden; transition: max-height 0.5s ease, opacity 0.35s ease; margin-top: 0; padding-left: 1rem;"
                }
            }>
                <div class="py-3 px-4 text-gray-700 bg-white">{children()}</div>
            </div>
        </div>
    }
}
