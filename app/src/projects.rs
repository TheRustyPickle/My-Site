use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use thaw::{
    Badge, BadgeAppearance, Button, ButtonAppearance, ButtonShape, Card, Dialog, DialogContent,
    DialogSurface, DialogTitle,
};

#[derive(Clone)]
struct Project {
    name: String,
    description: String,
    title_image: Option<String>,
    badges: Vec<String>,
    content: ProjectContent,
}

#[derive(Clone)]
struct ProjectContent {
    images: Option<Vec<String>>,
    content: String,
    source_link: String,
    demo_link: Option<String>,
    use_nav: bool,
}

#[component]
pub fn Projects() -> impl IntoView {
    let dialog_open = RwSignal::new(false);

    let (open_project, set_open_project) = signal(None);

    let project_list = get_project_list();

    let open_dialog = move |project| {
        dialog_open.set(true);
        set_open_project.set(Some(project));
    };

    view! {
        <Title text="Projects | Rusty Pickle" />
        <div class="w-full max-w-5xl mx-auto p-6">
            <h2 class="text-3xl font-bold text-center text-gray-800">"My Projects"</h2>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mt-6">
                <For
                    each=move || project_list.clone()
                    key=|project| project.name.clone()
                    children=move |project| {
                        let project_clone = project.clone();
                        view! {
                            <Card
                                class="!rounded-lg !shadow-lg h-90 bg-white overflow-hidden flex flex-col cursor-pointer transition-all hover:scale-105 hover:shadow-lg active:scale-95"
                                on:click=move |_| { open_dialog(project_clone.clone()) }
                            >
                                <img
                                    src=project.title_image.clone()
                                    class="w-full h-40 object-cover transition-all hover:brightness-75"
                                />

                                <div class="p-4 flex flex-col flex-grow">
                                    <h3 class="text-xl font-semibold text-gray-900">
                                        {project.name.clone()}
                                    </h3>
                                    <p class="text-gray-700 text-sm mt-2 flex-grow">
                                        {project.description.clone()}
                                    </p>

                                    <div class="flex flex-wrap gap-2 mt-3">
                                        <For
                                            each=move || project.badges.clone()
                                            key=|badge| badge.clone()
                                            children=move |badge| {
                                                view! {
                                                    <Badge appearance=BadgeAppearance::Outline>{badge}</Badge>
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                            </Card>
                        }
                    }
                />
            </div>
        </div>

        <Dialog open=dialog_open>
            <DialogSurface>
                <Show when=move || open_project.get().is_some() fallback=|| view! { "" }>
                    {move || view! { <ShowDialog project=open_project.get().unwrap() /> }}
                </Show>
            </DialogSurface>
        </Dialog>
    }
}

#[component]
fn show_dialog(project: Project) -> impl IntoView {
    let name = project.name.clone();
    let content = project.content.content.clone();
    let badges = project.badges.clone();
    let images = project.content.images.clone();
    let source_link = project.content.source_link.clone();
    let demo_link = project.content.demo_link.clone();
    let use_nav = project.content.use_nav;

    let nav_or_link = move |demo_link: String| {
        if !use_nav {
            view! {
                <a href=demo_link target="_blank">
                    <Button
                        appearance=ButtonAppearance::Primary
                        icon=icondata::BiLinkExternalRegular
                    >
                        "Live Demo"
                    </Button>
                </a>
            }
            .into_any()
        } else {
            view! {
                <Button
                    appearance=ButtonAppearance::Primary
                    icon=icondata::BiLinkExternalRegular
                    on:click=move |_| {
                        let navigate = use_navigate();
                        navigate(&demo_link, Default::default());
                    }
                >
                    "Live Demo"
                </Button>
            }
            .into_any()
        }
    };

    view! {
        <div class="w-full max-w-xl">
            <Carousel images=images />
        </div>

        <DialogContent class="flex flex-col gap-4">
            <div class="flex flex-wrap gap-2 mt-4 justify-center items-center">
                <For each=move || badges.clone() key=|badge| badge.clone() let:badge>
                    <Badge appearance=BadgeAppearance::Outline>{badge}</Badge>
                </For>
            </div>

            <DialogTitle class="text-xl font-bold flex justify-center items-center">
                {name}
            </DialogTitle>

            <div class="prose max-w-none justify-center text-center">{content}</div>

            <div class="flex flex-wrap gap-3 mt-4 justify-center items-center">
                <a href=source_link target="_blank">
                    <Button appearance=ButtonAppearance::Primary icon=icondata::AiGithubFilled>
                        "View Source"
                    </Button>
                </a>

                <Show when={
                    let demo = demo_link.clone();
                    move || demo.is_some()
                }>
                    {
                        let demo_link = demo_link.clone().unwrap();
                        nav_or_link(demo_link)
                    }
                </Show>
            </div>
        </DialogContent>
    }
    .into_any()
}

#[component]
fn Carousel(images: Option<Vec<String>>) -> impl IntoView {
    // Early return if no images
    if let Some(img_vec) = &images {
        if img_vec.is_empty() {
            return view! { <div class="text-center py-4">No images available</div> }.into_any();
        }
    } else {
        return view! { <div class="text-center py-4">No images available</div> }.into_any();
    }

    let images = images.unwrap();
    let (current_index, set_current_index) = signal(0);
    let (next_index, set_next_index) = signal(0);
    let is_animating = RwSignal::new(false);
    let (slide_direction, set_slide_direction) = signal("none");

    let total_images = images.len();

    // Reset carousel state after animation completes
    let reset_animation = move || {
        set_current_index.set(next_index.get());
        set_slide_direction.set("reset");

        set_timeout(
            move || {
                set_slide_direction.set("none");
                is_animating.set(false);
            },
            std::time::Duration::from_millis(50),
        );
    };

    // Move to next image
    let next = move |_| {
        if is_animating.get() {
            return;
        }

        let new_index = (current_index.get() + 1) % total_images;
        set_next_index.set(new_index);
        set_slide_direction.set("next");
        is_animating.set(true);

        set_timeout(reset_animation, std::time::Duration::from_millis(500));
    };

    let prev = move |_| {
        if is_animating.get() {
            return;
        }

        let new_index = if current_index.get() == 0 {
            total_images - 1
        } else {
            current_index.get() - 1
        };

        set_next_index.set(new_index);
        set_slide_direction.set("init-prev");

        is_animating.set(true);

        set_timeout(
            move || {
                set_slide_direction.set("prev");
            },
            std::time::Duration::from_millis(20),
        );

        set_timeout(reset_animation, std::time::Duration::from_millis(520));
    };

    view! {
        <div class="flex w-full w-3xl flex-1 justify-center item-center">

            <div class="carousel-root w-full ">
                // CSS for animations - scoped with unique class name
                <style>
                    {".carousel-root .carousel-container {
                    width: 100%;
                    max-width: 100%;
                    overflow: hidden;
                    position: relative;
                    height: 300px;
                    margin: 0 auto;
                    }
                    .carousel-root .carousel-track {
                    display: flex;
                    width: 200%;
                    height: 100%;
                    position: relative;
                    transition: transform 0.5s ease;
                    }
                    .carousel-root .carousel-slide {
                    width: 50%;
                    height: 100%;
                    flex-shrink: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    }
                    .carousel-root .carousel-slide img {
                    max-width: 100%;
                    max-height: 100%;
                    width: 100%;
                    height: 100%;
                    object-fit: contain;
                    }
                    .carousel-root .slide-none .carousel-track {
                    transform: translateX(0);
                    }
                    .carousel-root .slide-next .carousel-track {
                    transform: translateX(-50%);
                    }
                    .carousel-root .slide-prev .carousel-track {
                    transform: translateX(0);
                    }
                    .carousel-root .init-prev .carousel-track {
                    transform: translateX(-50%);
                    transition: none;
                    }
                    .carousel-root .slide-reset .carousel-track {
                    transition: none;
                    transform: translateX(0);
                    }
                    .carousel-root .image-counter {
                    margin-top: 0.5rem;
                    text-align: center;
                    font-size: 0.875rem;
                    color: #6b7280;
                    }"}
                </style>

                // Carousel main container
                <div
                    class="carousel-container "
                    class:slide-next=move || slide_direction.get() == "next"
                    class:slide-prev=move || slide_direction.get() == "prev"
                    class:slide-none=move || slide_direction.get() == "none"
                    class:slide-reset=move || slide_direction.get() == "reset"
                    class:init-prev=move || slide_direction.get() == "init-prev"
                >
                    // Track containing slides
                    <div class="carousel-track">
                        {move || {
                            let curr = current_index.get();
                            let next = next_index.get();
                            let direction = slide_direction.get();
                            match direction {
                                "next" => {
                                    view! {
                                        // Current slide on the left
                                        <div class="carousel-slide">
                                            <img
                                                src=images[curr].clone()
                                                alt=format!("Image {}", curr + 1)
                                            />
                                        </div>
                                        // Next slide on the right
                                        <div class="carousel-slide">
                                            <img
                                                src=images[next].clone()
                                                alt=format!("Image {}", next + 1)
                                            />
                                        </div>
                                    }
                                        .into_any()
                                }
                                "prev" | "init-prev" => {
                                    view! {
                                        // Previous slide on the left
                                        <div class="carousel-slide">
                                            <img
                                                src=images[next].clone()
                                                alt=format!("Image {}", next + 1)
                                            />
                                        </div>
                                        // Current slide on the right
                                        <div class="carousel-slide">
                                            <img
                                                src=images[curr].clone()
                                                alt=format!("Image {}", curr + 1)
                                            />
                                        </div>
                                    }
                                        .into_any()
                                }
                                _ => {
                                    view! {
                                        // Only current slide visible
                                        <div class="carousel-slide">
                                            <img
                                                src=images[curr].clone()
                                                alt=format!("Image {}", curr + 1)
                                            />
                                        </div>
                                        <div class="carousel-slide"></div>
                                    }
                                        .into_any()
                                }
                            }
                        }}
                    </div>

                    // Navigation arrows
                    <Show when=move || { total_images > 1 }>
                        <div class="inset-0 absolute z-10">
                            <Button
                                on:click=prev
                                disabled=is_animating
                                shape=ButtonShape::Circular
                                icon=icondata::FaChevronLeftSolid
                                class="opacity-50 absolute left-0 top-1/2 transform -translate-y-1/2 ml-1"
                            />

                            <Button
                                on:click=next
                                shape=ButtonShape::Circular
                                disabled=is_animating
                                icon=icondata::FaChevronRightSolid
                                class="opacity-50 absolute right-0 top-1/2 transform -translate-y-1/2 mr-1"
                            />
                        </div>
                    </Show>
                </div>

                // Image counter
                <div class="image-counter">
                    {move || format!("Image {} of {}", current_index.get() + 1, total_images)}
                </div>
            </div>
        </div>
    }
    .into_any()
}

fn get_project_list() -> Vec<Project> {
    let rex_content = ProjectContent {
        content: String::from("A tui program"),
        demo_link: None,
        images: Some(vec![String::from("https://fastly.picsum.photos/id/0/5000/3333.jpg?hmac=_j6ghY5fCfSD6tvtcV74zXivkJSPIfR9B8w34XeQmvU"), String::from("https://fastly.picsum.photos/id/2/5000/3333.jpg?hmac=_KDkqQVttXw_nM-RyJfLImIbafFrqLsuGO5YuHqD-qQ")]),
        source_link: String::from("https://github.com/TheRustyPickle/rex"),
        use_nav: false,
    };
    let rex = Project {
        title_image: Some(String::from("https://kzmgsz03dn2o1l269lcn.lite.vusercontent.net/placeholder.svg?height=600&width=800")),
        name: String::from("Rex"),
        description: String::from("A TUI program for keeping track of incomes and expenses"),
        badges: vec![
            "Rust".to_string(),
            "TUI".to_string(),
            "SQLite".to_string(),
        ],
        content: rex_content,
    };

    let talon_content = ProjectContent {
        content: String::from("A tui program"),
        demo_link: None,
        images: Some(vec![String::from("https://fastly.picsum.photos/id/0/5000/3333.jpg?hmac=_j6ghY5fCfSD6tvtcV74zXivkJSPIfR9B8w34XeQmvU"), String::from("https://fastly.picsum.photos/id/2/5000/3333.jpg?hmac=_KDkqQVttXw_nM-RyJfLImIbafFrqLsuGO5YuHqD-qQ")]),
        source_link: String::from("https://github.com/TheRustyPickle/Talon"),
        use_nav: false,
    };
    let talon = Project {
        title_image: Some(String::from("https://kzmgsz03dn2o1l269lcn.lite.vusercontent.net/placeholder.svg?height=600&width=800")),
        name: String::from("Talon"),
        description: String::from("A tool to generate on-demand data insights from public Telegram chats"),
        badges: vec![
            "Rust".to_string(),
            "GUI".to_string(),
            "Telegram".to_string(),
            "Analytics".to_string(),
        ],
        content: talon_content,
    };

    let funnel_content = ProjectContent {
        content: String::from("A tui program"),
        demo_link: Some(String::from("https://therustypickle.github.io/Funnel-Web/")),
        images: Some(vec![String::from("https://fastly.picsum.photos/id/0/5000/3333.jpg?hmac=_j6ghY5fCfSD6tvtcV74zXivkJSPIfR9B8w34XeQmvU")]),
        source_link: String::from("https://github.com/TheRustyPickle/Funnel-Web"),
        use_nav: false,
    };
    let funnel = Project {
        title_image: Some(String::from("https://kzmgsz03dn2o1l269lcn.lite.vusercontent.net/placeholder.svg?height=600&width=800")),
        name: String::from("Funnel"),
        description: String::from("A platform for visualizing Discord guild analytics"),
        badges: vec![
            "Rust".to_string(),
            "WASM".to_string(),
            "WebSocket".to_string(),
            "PostgreSQL".to_string(),
            "Discord".to_string(),
            "Actix-Web".to_string(),
        ],
        content: funnel_content,
    };

    let chirp_content = ProjectContent {
        content: String::from("A tui program"),
        demo_link: None,
        images: Some(vec![String::from("https://fastly.picsum.photos/id/0/5000/3333.jpg?hmac=_j6ghY5fCfSD6tvtcV74zXivkJSPIfR9B8w34XeQmvU")]),
        source_link: String::from("https://github.com/TheRustyPickle/Chirp"),
        use_nav: false,
    };
    let chirp = Project {
        title_image: Some(String::from("https://kzmgsz03dn2o1l269lcn.lite.vusercontent.net/placeholder.svg?height=600&width=800")),
        name: String::from("Chirp"),
        description: String::from("A chat app built from scratch using GTK4 and Rust with encryption"),
        badges: vec![
            "Rust".to_string(),
            "GTK4".to_string(),
            "Encryption".to_string(),
            "WebSocket".to_string(),
            "PostgreSQL".to_string(),
            "Diesel".to_string(),
        ],
        content: chirp_content,
    };

    let repo_dl_content = ProjectContent {
        content: String::from("A tui program"),
        demo_link: Some(String::from("/repo")),
        images: Some(vec![String::from("https://fastly.picsum.photos/id/0/5000/3333.jpg?hmac=_j6ghY5fCfSD6tvtcV74zXivkJSPIfR9B8w34XeQmvU")]),
        source_link: String::from("https://github.com/TheRustyPickle/rex"), // TODO: Update
        use_nav: true,
    };
    let repo_dl = Project {
        title_image: Some(String::from("https://kzmgsz03dn2o1l269lcn.lite.vusercontent.net/placeholder.svg?height=600&width=800")),
        name: String::from("Repo D/L"),
        description: String::from("Straightforward web app to view GitHub Repo release download status"),
        badges: vec![
            "Rust".to_string(),
            "Leptos".to_string(),
            "WASM".to_string(),
            "Octocrab".to_string(),
            "Actix-Web".to_string(),
        ],
        content: repo_dl_content,
    };

    let reddit_dl_content = ProjectContent {
        content: String::from("A tui program"),
        demo_link: Some(String::from("/reddit")),
        images: Some(vec![String::from("https://fastly.picsum.photos/id/0/5000/3333.jpg?hmac=_j6ghY5fCfSD6tvtcV74zXivkJSPIfR9B8w34XeQmvU")]),
        source_link: String::from("https://github.com/TheRustyPickle/rex"),
        use_nav: true,
    };

    let reddit_dl = Project {
        title_image: Some(String::from("https://kzmgsz03dn2o1l269lcn.lite.vusercontent.net/placeholder.svg?height=600&width=800")),
        name: String::from("Redit D/L"),
        description: String::from("A simple web app to download content from a reddit post"),
        badges: vec![
            "Rust".to_string(),
            "Leptos".to_string(),
            "Ruox".to_string(),
            "Dash-MPD".to_string(),
            "Actix-Web".to_string(),
        ],
        content: reddit_dl_content,
    };

    let selectable_table_content = ProjectContent {
        content: String::from("A tui program"),
        demo_link: Some(String::from("https://therustypickle.github.io/egui-selectable-table/")),
        images: Some(vec![String::from("https://fastly.picsum.photos/id/0/5000/3333.jpg?hmac=_j6ghY5fCfSD6tvtcV74zXivkJSPIfR9B8w34XeQmvU")]),
        source_link: String::from("https://github.com/TheRustyPickle/egui-selectable-table"),
        use_nav: true,
    };

    let selectable_table = Project {
        title_image: Some(String::from("https://kzmgsz03dn2o1l269lcn.lite.vusercontent.net/placeholder.svg?height=600&width=800")),
        name: String::from("egui Selectable Table"),
        description: String::from("A library for egui to create tables with draggable cell and row selection."),
        badges: vec![
            "Rust".to_string(),
            "egui".to_string(),
            "Library".to_string(),
            "Widget".to_string(),
        ],
        content: selectable_table_content,
    };

    let theme_lerp_content = ProjectContent {
        content: String::from("A tui program"),
        demo_link: Some(String::from(
            "https://therustypickle.github.io/egui-theme-lerp/",
        )),
        images: None,
        source_link: String::from("https://github.com/TheRustyPickle/egui-theme-lerp"),
        use_nav: true,
    };

    let theme_lerp = Project {
        title_image: Some(String::from(
            "https://kzmgsz03dn2o1l269lcn.lite.vusercontent.net/placeholder.svg",
        )),
        name: String::from("egui Theme Animation"),
        description: String::from(
            "A simple library for egui to smoothly animate theme transitions",
        ),
        badges: vec![
            "Rust".to_string(),
            "egui".to_string(),
            "Library".to_string(),
            "Animation".to_string(),
        ],
        content: theme_lerp_content,
    };

    let pulse_content = ProjectContent {
        content: String::from("A tui program"),
        demo_link: None,
        images: None,
        source_link: String::from("https://github.com/TheRustyPickle/Pulse"),
        use_nav: false,
    };

    let pulse = Project {
        title_image: Some(String::from(
            "https://kzmgsz03dn2o1l269lcn.lite.vusercontent.net/placeholder.svg",
        )),
        name: String::from("Pulse"),
        description: String::from(
            "A Discord bot for scheduling messages with simple configuration",
        ),
        badges: vec!["Rust".to_string(), "Discord".to_string(), "Bot".to_string()],
        content: pulse_content,
    };

    let this_site_content = ProjectContent {
        content: String::from("A tui program"),
        demo_link: Some(String::from("https://therustypickle.github.io/Funnel-Web/")), // TODO: Update
        images: None,
        source_link: String::from("https://github.com/TheRustyPickle/rex"),
        use_nav: false,
    };

    let this_site = Project {
        title_image: Some(String::from(
            "https://kzmgsz03dn2o1l269lcn.lite.vusercontent.net/placeholder.svg",
        )),
        name: String::from("This Site"),
        description: String::from("Info about this site"),
        badges: vec![
            "Rust".to_string(),
            "Leptos".to_string(),
            "Actix-Web".to_string(),
            "Thaw".to_string(),
        ],
        content: this_site_content,
    };

    vec![
        rex,
        talon,
        funnel,
        chirp,
        repo_dl,
        reddit_dl,
        selectable_table,
        theme_lerp,
        pulse,
        this_site,
    ]
}
