use api::message_drop;
use icondata::Icon;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::Title;
use thaw::{Button, ButtonAppearance, ButtonShape, ButtonSize, Card, Textarea};

#[derive(Clone)]
struct Social {
    icon: Icon,
    name: String,
    link: String,
}

#[component]
pub fn About() -> impl IntoView {
    let social_buttons = [
        Social {
            icon: icondata::FaGithubBrands,
            name: "GitHub".to_string(),
            link: "https://github.com/TheRustyPickle".to_string(),
        },
        Social {
            icon: icondata::FaDiscordBrands,
            name: "Discord".to_string(),
            link: "https://discord.com/users/406917444381179905".to_string(),
        },
        Social {
            icon: icondata::TbBrandTelegram,
            name: "Telegram".to_string(),
            link: "https://t.me/RustyPickle".to_string(),
        },
        Social {
            icon: icondata::MdiEmailOutline,
            name: "Email".to_string(),
            link: "mailto:rusty.pickle94@gmail.com".to_string(),
        },
    ];

    let p_1 = "I'm a hobbyist programmer who enjoys building projects that I find useful and contributing to open source.";

    let p_2 = "My background is in marketing, but programming has been a big part of what I do, \
        whether through personal projects or small tools I've made along the way.";

    let p_3 = "I love experimenting with new technologies and hope to transition into software development in the future. \
        While I've worked with a few languages, Rust has been my favorite for its reliability and performance.";

    let p_4 = "I'm always up for a chat about programming, \
        open-source, or whatever interesting project \
        you're working on—feel free to reach out!";

    view! {
        <Title text="About | Rusty Pickle" />
        <div class="flex flex-col gap-2 p-4">

            <Card class="w-full bg-white dark:bg-gray-800 !rounded-lg flex !gap-2 flex-col text-center">
                <h2 class="text-2xl font-bold text-gray-800 dark:text-gray-200">"About Me"</h2>
                <p class="mt-2 text-gray-600 dark:text-gray-300 text-lg">{p_1}</p>
                <p class="mt-2 text-gray-600 dark:text-gray-300 text-lg">{p_2}</p>
                <p class="mt-2 text-gray-600 dark:text-gray-300 text-lg">{p_3}</p>
                <p class="mt-2 text-gray-600 dark:text-gray-300 text-lg">{p_4}</p>
            </Card>

            <Card class="w-full flex flex-col bg-white dark:bg-gray-800 !rounded-lg">
                <div class="flex flex-wrap justify-center">
                    <For
                        each=move || social_buttons.clone()
                        key=|social| social.name.clone()
                        children=move |social| {
                            view! {
                                <a href=social.link.clone() target="_blank">
                                    <Button
                                        icon=social.icon
                                        shape=ButtonShape::Rounded
                                        appearance=ButtonAppearance::Transparent
                                        size=ButtonSize::Large
                                        class="!transition-all !duration-200 hover:scale-105 relative hover:z-10"
                                    >
                                        {social.name.clone()}
                                    </Button>
                                </a>
                            }
                        }
                    />
                </div>
            </Card>
            <MessageBox />
        </div>
    }
}

#[component]
pub fn MessageBox() -> impl IntoView {
    let message = RwSignal::new(String::new());
    let submitted = RwSignal::new(false);

    let send_message = move |text| {
        submitted.set(true);
        spawn_local(async move {
            let _ = message_drop(text).await;
        });
    };

    view! {
        <Card class="w-full bg-white dark:bg-gray-800 !rounded-lg flex flex-col gap-4 p-4 sm:p-6">
            <h3 class="text-xl font-semibold text-center text-gray-800 dark:text-gray-200">
                Drop me a message
            </h3>

            <Show
                when=move || !submitted.get()
                fallback=move || {
                    view! {
                        <p class="text-center text-green-600 dark:text-green-400">
                            Thanks! Your message has been sent.
                        </p>
                    }
                }
            >
                <Textarea
                    placeholder="Your message"
                    class="h-20"
                    value=message
                    on:input=move |ev| message.set(event_target_value(&ev))
                />
                <Button
                    on:click=move |_| {
                        if !message.get().is_empty() {
                            send_message(message.get().clone());
                        }
                    }
                    class="self-center w-full sm:w-1/2"
                >
                    Send
                </Button>
            </Show>
        </Card>
    }
}
