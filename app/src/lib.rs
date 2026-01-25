mod about;
mod projects;
mod reddit_dl;
mod repo_dl;
mod secrets;
mod utils;

use about::About;
use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::{StaticSegment, WildcardSegment, path};
use projects::Projects;
use reddit_dl::RedditDL;
use repo_dl::RepoDL;
use secrets::Secrets;
use std::collections::HashMap;
use thaw::{ConfigProvider, Layout, LayoutPosition, Tab, TabList, Theme};
use web_sys::MediaQueryListEvent;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::Closure;

#[component]
#[must_use]
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
        let brand_colors = brand_colors.get_untracked();
        theme.set(Theme::custom_light(&brand_colors));
    };

    let tab_value = RwSignal::new(String::new());

    let set_tab_value = move || {
        let location = use_location();
        move || {
            let path = location.pathname.get();
            tab_value.set(match path.as_str() {
                "/about" => "about".to_string(),
                "/" | "/projects" => "projects".to_string(),
                _ => String::new(),
            });
        }
    };

    let navigate_to_page = move |_| {
        let selected_value = tab_value.get();
        let value_path = match selected_value.as_str() {
            "projects" => "/projects",
            "about" => "/about",
            _ => "/not_found",
        };

        let navigate = use_navigate();
        navigate(value_path, Default::default());
    };

    let (style_color, set_style) = signal(String::from("f3f4f6ff"));

    Effect::new(move |_| {
        let media_query = window()
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .unwrap();

        if media_query.matches() {
            theme.set(Theme::dark());
            set_style.set(String::from("111827ff"));
        } else {
            on_customize_light_theme();
        }

        let callback = Closure::wrap(Box::new(move |event: MediaQueryListEvent| {
            if event.matches() {
                theme.set(Theme::dark());
                set_style.set("111827ff".to_string());
            } else {
                on_customize_light_theme();
            }
        }) as Box<dyn FnMut(_)>);

        media_query
            .add_listener_with_opt_callback(Some(callback.as_ref().unchecked_ref()))
            .unwrap();

        callback.forget();
    });

    let computed_style = move || format!("background-color: #{};", style_color.get());

    view! {
        <Stylesheet id="leptos" href="/pkg/my_site.css" />
        <script async src="https://scripts.simpleanalyticscdn.com/latest.js"></script>
        <Title text="Rusty Pickle" />

        <Router>
            {set_tab_value()} <main>
                <ConfigProvider theme>
                    <Layout position=LayoutPosition::Absolute attr:style=computed_style>
                        <div class="flex justify-center item-center mt-1 bg-gray-100 dark:bg-gray-900">
                            <TabList
                                selected_value=tab_value
                                class="min-w-30 mb-8 bg-white dark:bg-gray-800 justify-center item-center flex rounded-lg"
                            >
                                <Tab value="projects" on:click=navigate_to_page>
                                    "Projects"
                                </Tab>
                                <Tab value="about" on:click=navigate_to_page>
                                    "About"
                                </Tab>
                            </TabList>
                        </div>
                        <div class="bg-gray-100 dark:bg-gray-900">
                            <Routes fallback=move || NotFound>
                                <Route path=StaticSegment("") view=ToProjectPage />
                                <Route path=StaticSegment("/projects") view=Projects />
                                <Route path=StaticSegment("/reddit") view=RedditDL />
                                <Route path=StaticSegment("/repo") view=RepoDL />
                                <Route path=StaticSegment("/about") view=About />
                                <Route path=path!("/secrets/:id") view=Secrets />
                                <Route path=WildcardSegment("any") view=NotFound />
                            </Routes>
                        </div>
                    </Layout>
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
fn ToProjectPage() -> impl IntoView {
    let navigate = use_navigate();

    Effect::new(move |_| {
        navigate("/projects", Default::default());
    });
}
