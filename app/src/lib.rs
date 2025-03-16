mod test;
mod about;
mod projects;
mod reddit_dl;
mod repo_dl;
mod utils;

use about::About;
use test::TestPage;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::{StaticSegment, WildcardSegment};
use projects::Projects;
use reddit_dl::RedditDL;
use repo_dl::RepoDL;
use std::collections::HashMap;
use thaw::{ConfigProvider, Tab, TabList, Theme};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let theme = RwSignal::new(Theme::light());

    let brand_colors = RwSignal::new(HashMap::from([
        (10, "#F2F9FF"),
        (20, "#E0F2FF"),
        (30, "#C9E6FF"),
        (40, "#A8D4FF"),
        (50, "#85C1FF"),
        (60, "#66AEFF"),
        (70, "#4A9BFF"),
        (80, "#3187FF"),
        (90, "#2074E6"),
        (100, "#1A66CC"),
        (110, "#1557B3"),
        (120, "#104899"),
        (130, "#0B3A80"),
        (140, "#072B66"),
        (150, "#041D4D"),
        (160, "#021133"),
    ]));

    let on_customize_light_theme = move || {
        theme.set(Theme::custom_light(&brand_colors.get_untracked()));
    };

    on_customize_light_theme();

    let tab_value = RwSignal::new(String::new());

    let set_tab_value = move || {
        let location = use_location();
        move || {
            let path = location.pathname.get();
            tab_value.set(match path.as_str() {
                "/reddit" => "reddit".to_string(),
                "/repo" => "repo".to_string(),
                "/about" => "about".to_string(),
                "/" | "/projects" => "projects".to_string(),
                _ => "projects".to_string(),
            });
        }
    };

    let navigate_to_page = move |_| {
        let selected_value = tab_value.get();
        let value_path = if selected_value == "projects" {
            "/projects"
        } else if selected_value == "reddit" {
            "/reddit"
        } else if selected_value == "repo" {
            "/repo"
        } else if selected_value == "about" {
            "/test"
        } else {
            "/not_found"
        };

        let navigate = use_navigate();
        navigate(value_path, Default::default());
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/dl_reddit.css" />
        <Title text="Rusty Pickle" />

        <Router>
            {set_tab_value()} <main>
                <ConfigProvider theme>
                    <div class="flex justify-center item-center mt-1 bg-gray-100">
                        <TabList
                            selected_value=tab_value
                            class="min-w-72 mb-8 bg-white justify-center item-center flex rounded-lg"
                        >
                            <Tab value="projects" on:click=navigate_to_page>
                                "Projects"
                            </Tab>
                            <Tab value="reddit" on:click=navigate_to_page>
                                "Reddit D/L"
                            </Tab>
                            <Tab value="repo" on:click=navigate_to_page>
                                "Repo D/L"
                            </Tab>
                            <Tab value="about" on:click=navigate_to_page>
                                "About"
                            </Tab>
                        </TabList>
                    </div>
                    <div class="bg-gray-100">
                        <Routes fallback=move || "Not found.">
                            <Route path=StaticSegment("") view=ToProjectPage />
                            <Route path=StaticSegment("/projects") view=Projects />
                            <Route path=StaticSegment("/reddit") view=RedditDL />
                            <Route path=StaticSegment("/repo") view=RepoDL />
                            <Route path=StaticSegment("/about") view=About />
                            <Route path=StaticSegment("/test") view=TestPage />
                            <Route path=WildcardSegment("any") view=NotFound />
                        </Routes>
                    </div>
                </ConfigProvider>
            </main>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Title text="Not Found | Rusty Pickle" />
        <div class="text-2xl pt-5 justify-center item-center flex">"Not Found"</div>
    }
}

#[component]
fn ToProjectPage() {
    let navigate = use_navigate();

    Effect::new(move |_| {
        navigate("/projects", Default::default());
    });
}
