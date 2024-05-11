use leptos::*;


fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[component]
fn App()->impl IntoView{
    let (count,set_count) = create_signal(0);
    let double_count = move || count()*2;
    let (value,set_value) = create_signal("B".to_string());
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


        //<DynamicComplexList />


        //<ControlledInput />


        //<UncontrolledInput />


        // <select on:change=move |ev| {
        //             let new_value = event_target_value(&ev);
        //             set_value(new_value);
        //         }>
        //             <SelectOption value is="A"/>
        //             <SelectOption value is="B"/>
        //             <SelectOption value is="C"/>
        //         </select>
        //         <p>{value}</p>

        <IfComponentStatement />
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
    value: RwSignal<isize>,
}

#[component]
fn DynamicComplexList()->impl IntoView{
    let (data,set_data) = create_signal(vec![
        DatabaseEntry{
            key:"foo".to_string(),
            value: create_rw_signal(1)
        },
        DatabaseEntry{
            key:"bar".to_string(),
            value: create_rw_signal(2)
        },
        DatabaseEntry{
            key:"man".to_string(),
            value: create_rw_signal(3)
        }
    ]);
        view! {
            <button on:click=move |_|{
                set_data.update(|data|{
                    for row in data{
                        row.value.update(|value| *value*=2)
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

#[component]
fn ControlledInput()->impl IntoView{
    let (name,set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
        on:input=move |ev|{
            set_name(event_target_value(&ev));
        }
        prop:value=name
        />
        <p>Name is {name}</p>
    }
}

#[component]
fn UncontrolledInput()->impl IntoView{
    let (name,set_name) = create_signal("uncontrolled".to_string());
    let input_element:NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let value = input_element()
            .expect("<input> should be mounted")
            .value();
        set_name(value);
    };
    view! {
    <form on:submit=on_submit>
        <input type="text"
            value=name
            node_ref=input_element
        />
        <input type="submit" value="Submit"/>
    </form>
    <p>"Name is: " {name}</p>
    }
}

#[component]
fn SelectOption(is: &'static str,value:ReadSignal<String>)->impl IntoView{
    view! {
        <option
            value=is
            selected=move || value() == is
        >
            {is}
        </option>
    }
}

#[component]
fn IfComponentStatement()->impl IntoView{
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;
    //let message = move || if is_odd() {
    //         Some("Oh yeah it's odd")
    //     }else {
    //         None
    //     };

    let message = move || {
    match value() {
        0 => "Zero",
        1 => "One",
        n if is_odd() => "Odd",
        _ => "Even"
    }
};
    view! {
        <p>
        {message}
        </p>
    }
}