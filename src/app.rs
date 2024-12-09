use leptos::*;
use leptos_meta::*;

use crate::models::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let messages = vec![
        Message::new("Hello".to_string(), false),
        Message::new("Hi".to_string(), true),
    ];
    let messages_2 = vec![
        Message::new("Lollllllllllllll".to_string(), false),
        Message::new("Bruuuuuhhhhhhhhhhhh".to_string(), true),
    ];
    let conversations = vec![
        Conversation::new(
            "ChatterPatter".to_string(),
            "Ram".to_string(),
            "Shyam".to_string(),
            messages,
        ),
        Conversation::new(
            "Patter Chatter".to_string(),
            "Kishor".to_string(),
            "Kumar".to_string(),
            messages_2,
        ),
    ];

    let (conversations, set_conversations) = create_signal(conversations);
    let (active_conversation, set_active_conversation) = create_signal(Conversation::default());

    let handle_set_active_conversation = move |id: String| {
        let conv =
            conversations.with(|convos| convos.iter().find(|c| c.get_id() == id).cloned().unwrap());
        logging::log!("{:?}", conv);
        set_active_conversation.set(conv);
    };

    let handle_new_message = move |msg: String| {
        let message = Message::new(msg, false);
        if active_conversation.with(|conv| conv.get_id().to_owned()) != String::default() {
            set_active_conversation.update(|prev| prev.new_message(message));
        } else {
            let c = Conversation::new(
                message.get_content().to_owned(),
                "Client".to_string(),
                "server".to_string(),
                vec![message],
            );
            let new_conversation_id = c.get_id().to_owned();
            set_conversations.update(|prev| prev.insert(0, c));
            handle_set_active_conversation(new_conversation_id);
        }
        logging::log!("{:?}", active_conversation.get());
    };

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-fullstack.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        <div class="main-app border-1">
            <div class="top-half">
                <Sidebar
                    conversations=conversations
                    handle_conversation_clicked=handle_set_active_conversation
                />
                <ConversationArea conversation=active_conversation />
            </div>
            <MessageArea on_new_message=handle_new_message />
        </div>
        <Apppp />
    }
}

#[component]
pub fn Sidebar<F>(
    #[prop(into)] conversations: ReadSignal<Vec<Conversation>>,
    handle_conversation_clicked: F,
) -> impl IntoView
where
    F: Fn(String) + Copy + 'static,
{
    view! {
        <div class="sidebar">
            <h3>"Conversations"</h3>
            <ul>
                <For each=move || conversations.get() key=|c| c.get_id().to_owned() let:child>
                    {
                        let title = child.get_title().to_owned();
                        let id = child.get_id().to_owned();
                        view! {
                            <li on:click=move |_| handle_conversation_clicked(
                                id.clone(),
                            )>{title}</li>
                        }
                    }
                </For>
            </ul>
        </div>
    }
}

#[component]
pub fn ConversationArea(#[prop(into)] conversation: ReadSignal<Conversation>) -> impl IntoView {
    let messages_memo =
        Memo::new(move |_| conversation.with(|conv| conv.get_messages().to_owned()));
    view! {
        <div class="conversation-area">
            <Show
                when=move || !messages_memo.get().is_empty()
                fallback=|| {
                    view! { <h3>"Hello, Enter some message"</h3> }
                }
            >
                <div class="message-container">
                    <For
                        each=move || { messages_memo.get().into_iter().enumerate() }
                        key=|(_, m)| m.get_id().to_owned()
                        let:child
                    >
                        {
                            let message = child.1;
                            let from_server = message.get_from_server();
                            view! {
                                <div class=move || {
                                    if from_server { "border-1 server" } else { "border-1 client" }
                                }>{message.get_content().to_owned()}</div>
                            }
                        }
                    </For>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn MessageArea(on_new_message: impl Fn(String) + 'static) -> impl IntoView {
    let (new_message, set_new_message) = create_signal("".to_string());
    view! {
        <hr />
        <div class="message-area">
            <input
                type="text"
                on:input=move |ev| set_new_message.set(event_target_value(&ev))
                prop:value=new_message
            />
            <button on:click=move |_| {
                on_new_message(new_message.get());
                set_new_message.set("".into())
            }>"Send"</button>
        </div>
    }
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn Apppp() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            set_data
                .update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
            leptos::logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        // iterate over the rows and display each value
        <For
            each=move || data.get().into_iter().enumerate()
            key=|(_, state)| state.key.clone()
            children=move |(index, _)| {
                let value = Memo::new(move |_| {
                    data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                });
                view! { <p>{value}</p> }
            }
        />
    }
}
