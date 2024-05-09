use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App />})
}

#[component]
fn App()->impl IntoView{
    let (count,set_count) = create_signal(0);
    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n +=1)
            }
            class:red=move || count() % 2==1
            style="position: absolute"
                //style:left=move || format!("{}px", count() + 100)
                //style:background-color=move || format!("rgb({}, {}, 100)", count(), 100)
                //style:min-width="400px"
                //style=("--columns", count)
        >
            "Click me: "
            {count}
        </button>
        <progress
            max="100"
        class="middle"
            value=count
        />
    }
}