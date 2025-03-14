use icondata::Icon;
use leptos::prelude::*;
use leptos_meta::Title;
use thaw::{Avatar, Button, ButtonAppearance, ButtonShape, Card};

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
            icon: icondata::AiGithubOutlined,
            name: "GitHub".to_string(),
            link: "https://github.com/TheRustyPickle".to_string(),
        },
        Social {
            icon: icondata::BsDiscord,
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
    view! {
        <Title text="About | Rusty Pickle" />
        <div class="flex flex-col md:flex-row justify-center items-center gap-6 p-4 w-full max-w-4xl mx-auto p-10">

            <Card class="w-full md:w-1/2 p-6 flex flex-col items-center bg-white rounded-lg shadow-md min-h-61">
                <p class="font-bold text-3xl">Contact</p>
                <Avatar src="https://avatars.githubusercontent.com/u/35862475?v=4" size=96 />

                <div class="flex flex-wrap justify-center item-center">
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

            <Card class="w-full md:w-1/2 bg-white rounded-lg shadow-md flex flex-col justify-center item-center text-center min-h-61">
                <h2 class="text-2xl font-bold text-gray-800">"About Me"</h2>
                <p class="mt-2 text-gray-600 text-lg">
                    "I'm a Rust developer passionate about building efficient and scalable applications.
                    I love open-source projects and enjoy experimenting with new technologies."
                </p>
            </Card>
        </div>
    }
}
