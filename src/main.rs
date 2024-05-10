use leptos::*;


fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[component]
fn App()->impl IntoView{
    let (count,set_count) = create_signal(0);
    let double_count = move || count()*2;
    view! {
        //<button
        //             on:click=move |_| {
        //                 set_count.update(|n| *n +=1)
        //             }
        //             class:red=move || count() % 2==1
        //             style="position: absolute"
        //                 //style:left=move || format!("{}px", count() + 100)
        //                 //style:background-color=move || format!("rgb({}, {}, 100)", count(), 100)
        //                 //style:min-width="400px"
        //                 //style=("--columns", count)
        //         >
        //             "Click me: "
        //             {count}
        //         </button>
        //<ProgressBar max=10 progress=count />x
        //<ProgressBar max=10 progress=double_count />x
        //<List/>
        //<DynamicList initial_length=5 />
        <DynamicComplexList />
    }
}

#[component]
fn ProgressBar<F:Fn() -> i32 + 'static,>(
    #[prop(default=100)]
    max: usize,
    progress: F
)->impl IntoView {
    view! {
        <progress
            class="middle"
            max=max
            value=progress
        />
    }
}

#[component]
fn List()->impl IntoView{
    let values = vec![12,14,16,18];
    view! {
        <p>{values.clone()}</p>
        <span>clone function</span>

        <ul>
        {values.into_iter()
            .map(|n| view! {<li>{n}</li>})
            //.collect::<Vec<_>>()
            .collect_view()}
        </ul>
        <span>mapping function</span>
    }
}

#[component]
fn DynamicList(
    initial_length: usize,
) -> impl IntoView {

    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig))
        });
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>

                <For

                    each=counters

                    key=|counter| counter.0

                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, (signal, _))| {

                                                if counter_id == &id {
                                                    signal.dispose();
                                                }
                                                counter_id != &id
                                            })
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
fn DynamicComplexList()->impl IntoView{
    let (data,set_data) = create_signal(vec![
        DatabaseEntry{
            key:"foo".to_string(),
            value: 10
        },
        DatabaseEntry{
            key:"bar".to_string(),
            value: 34
        },
        DatabaseEntry{
            key:"man".to_string(),
            value: 45
        }
    ]);
        view! {
            <button on:click=move |_|{
                set_data.update(|data|{
                    for row in data{
                        row.value*=2;
                    }
                });
                logging::log!("{:?}",data.get())
            }>
            "update values"
            </button>
            <For
            each=data
            key=|state| state.key.clone()
            let:child
            >
            <p>{child.value}</p>
            </For>

        }
}