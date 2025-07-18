use leptos::prelude::*;
use leptos_meta::Title;
use thaw::{
    Badge, BadgeAppearance, Button, ButtonAppearance, ButtonShape, Card, Dialog, DialogContent,
    DialogSurface, DialogTitle, Scrollbar,
};

#[derive(Copy, Clone)]
enum ContentProject {
    Rex,
    Talon,
    Funnel,
    Chirp,
    RepoDL,
    RedditDL,
    Table,
    Theme,
    Pulse,
}

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
    content: ContentProject,
    source_link: String,
    demo_link: Option<String>,
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
        <div class="w-full max-w-5xl mx-auto p-4">
            <h2 class="text-3xl font-bold text-center text-gray-800 dark:text-gray-200">
                "My Projects"
            </h2>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mt-6">
                <For
                    each=move || project_list.clone()
                    key=|project| project.name.clone()
                    children=move |project| {
                        let project_clone = project.clone();
                        view! {
                            <Card
                                class="!rounded-lg !shadow-lg h-90 bg-white dark:bg-gray-800 overflow-hidden flex flex-col cursor-pointer transition-all hover:scale-105 hover:shadow-lg active:scale-95"
                                on:click=move |_| { open_dialog(project_clone.clone()) }
                            >
                                <img
                                    src=project.title_image.clone()
                                    class="w-full h-40 object-cover transition-all hover:brightness-75"
                                />

                                <div class="p-4 flex flex-col flex-grow">
                                    <h3 class="text-xl font-semibold text-gray-900 dark:text-gray-100">
                                        {project.name.clone()}
                                    </h3>
                                    <p class="text-gray-700 dark:text-gray-300 text-sm mt-2 flex-grow">
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
                <Show when=move || {
                    open_project.get().is_some()
                }>
                    {move || {
                        view! { <ShowDialog project=open_project.get().unwrap() dialog_open /> }
                    }}
                </Show>
            </DialogSurface>
        </Dialog>
    }
}

#[component]
fn show_dialog(project: Project, dialog_open: RwSignal<bool>) -> impl IntoView {
    let name = project.name.clone();
    let content = project.content.content;
    let badges = project.badges.clone();
    let images = project.content.images.clone();
    let source_link = project.content.source_link.clone();
    let demo_link = project.content.demo_link.clone();

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

            <Scrollbar style="max-height: 200px;" class="prose max-w-none overflow-y-auto">
                {get_project_content(content)}
            </Scrollbar>

            <div class="flex flex-wrap gap-3 mt-4 justify-center items-center">
                <a href=source_link target="_blank">
                    <Button appearance=ButtonAppearance::Primary icon=icondata::FaGithubBrands>
                        "View Source"
                    </Button>
                </a>

                <Show when={
                    let demo = demo_link.clone();
                    move || demo.is_some()
                }>
                    {
                        let demo_link = demo_link.clone().unwrap();
                        let a_target = if demo_link.starts_with("/") { "" } else { "_blank" };
                        view! {
                            <a href=demo_link target=a_target>
                                <Button
                                    appearance=ButtonAppearance::Primary
                                    icon=icondata::MdiLinkVariant
                                >
                                    "Live Demo"
                                </Button>
                            </a>
                        }
                    }
                </Show>

                <Button
                    appearance=ButtonAppearance::Primary
                    icon=icondata::MdiClose
                    on:click=move |_| dialog_open.set(false)
                >
                    "Close"
                </Button>
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
    // Add a zoom state signal
    let (is_zoomed, set_is_zoomed) = signal(false);

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
        // Don't allow navigation while zoomed or animating
        if is_animating.get() || is_zoomed.get() {
            return;
        }

        let new_index = (current_index.get() + 1) % total_images;
        set_next_index.set(new_index);
        set_slide_direction.set("next");
        is_animating.set(true);

        set_timeout(reset_animation, std::time::Duration::from_millis(500));
    };

    let prev = move |_| {
        // Don't allow navigation while zoomed or animating
        if is_animating.get() || is_zoomed.get() {
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

    // Toggle zoom handler
    let toggle_zoom = move |_| {
        // Don't allow zoom while animating
        if !is_animating.get() {
            set_is_zoomed.update(|zoomed| *zoomed = !*zoomed);
        }
    };

    view! {
        <div class="flex w-full w-3xl flex-1 justify-center item-center">

            <div class="carousel-root w-full">
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
                    transition: transform 0.3s ease;
                    cursor: zoom-in;
                    }
                    
                    .carousel-root .carousel-slide img.zoomed {
                    transform: scale(1.5);
                    cursor: zoom-out;
                    }
                    
                    .carousel-root .carousel-slide img:hover {
                    transform: scale(1.1);
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
                    }
                    
                    .carousel-root .zoom-overlay {
                    position: fixed;
                    top: 0;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    background-color: rgba(0, 0, 0, 0.75);
                    z-index: 20;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    }
                    
                    .carousel-root .zoom-overlay img {
                    max-width: 90%;
                    max-height: 90%;
                    object-fit: contain;
                    cursor: zoom-out;
                    }"}
                </style>

                // Carousel main container
                <div
                    class="carousel-container"
                    class:slide-next=move || slide_direction.get() == "next"
                    class:slide-prev=move || slide_direction.get() == "prev"
                    class:slide-none=move || slide_direction.get() == "none"
                    class:slide-reset=move || slide_direction.get() == "reset"
                    class:init-prev=move || slide_direction.get() == "init-prev"
                >
                    // Track containing slides
                    <div class="carousel-track">
                        {
                            let images = images.clone();
                            move || {
                                let curr = current_index.get();
                                let next = next_index.get();
                                let direction = slide_direction.get();
                                let zoomed = is_zoomed.get();
                                match direction {
                                    "next" => {
                                        view! {
                                            // Current slide on the left
                                            <div class="carousel-slide">
                                                <img
                                                    src=images[curr].clone()
                                                    alt=format!("Image {}", curr + 1)
                                                    on:click=toggle_zoom
                                                    class:zoomed=zoomed
                                                />
                                            </div>
                                            // Next slide on the right
                                            <div class="carousel-slide">
                                                <img
                                                    src=images[next].clone()
                                                    alt=format!("Image {}", next + 1)
                                                    on:click=toggle_zoom
                                                    class:zoomed=zoomed
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
                                                    on:click=toggle_zoom
                                                    class:zoomed=zoomed
                                                />
                                            </div>
                                            // Current slide on the right
                                            <div class="carousel-slide">
                                                <img
                                                    src=images[curr].clone()
                                                    alt=format!("Image {}", curr + 1)
                                                    on:click=toggle_zoom
                                                    class:zoomed=zoomed
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
                                                    on:click=toggle_zoom
                                                    class:zoomed=zoomed
                                                />
                                            </div>
                                            <div class="carousel-slide"></div>
                                        }
                                            .into_any()
                                    }
                                }
                            }
                        }
                    </div>

                    <Show when=move || is_zoomed.get()>
                        <div class="zoom-overlay" on:click=toggle_zoom>
                            <img
                                src={
                                    let image = images.clone();
                                    move || image[current_index.get()].clone()
                                }
                                alt=move || format!("Image {} (zoomed)", current_index.get() + 1)
                            />
                        </div>
                    </Show>
                </div>

                <Show when=move || { total_images > 1 && !is_zoomed.get() }>
                    <div class="flex justify-center items-center gap-6 mt-4">
                        <Button
                            on:click=prev
                            disabled=is_animating
                            shape=ButtonShape::Circular
                            icon=icondata::FaChevronLeftSolid
                            class="opacity-80 hover:opacity-100 transition-opacity"
                        />

                        <div class="image-counter font-medium">
                            {move || {
                                format!("Image {} of {}", current_index.get() + 1, total_images)
                            }}
                        </div>

                        <Button
                            on:click=next
                            shape=ButtonShape::Circular
                            disabled=is_animating
                            icon=icondata::FaChevronRightSolid
                            class="opacity-80 hover:opacity-100 transition-opacity"
                        />
                    </div>
                </Show>

            </div>
        </div>
    }
    .into_any()
}

fn get_project_list() -> Vec<Project> {
    let rex_content = ProjectContent {
        content: ContentProject::Rex,
        demo_link: None,
        images: Some(vec![
            String::from("/assets/rex_1.png"),
            String::from("/assets/rex_2.png"),
            String::from("/assets/rex_3.png"),
            String::from("/assets/rex_4.png"),
            String::from("/assets/rex_5.png"),
        ]),
        source_link: String::from("https://github.com/TheRustyPickle/rex"),
    };
    let rex = Project {
        title_image: Some(String::from("/assets/rex_1.png")),
        name: String::from("Rex"),
        description: String::from("A TUI program for keeping track of incomes and expenses"),
        badges: vec!["Rust".to_string(), "TUI".to_string(), "SQLite".to_string()],
        content: rex_content,
    };

    let talon_content = ProjectContent {
        content: ContentProject::Talon,
        demo_link: None,
        images: Some(vec![
            String::from("/assets/talon.png"),
            String::from("/assets/talon_1.png"),
            String::from("/assets/talon_2.png"),
            String::from("/assets/talon_3.png"),
            String::from("/assets/talon_4.png"),
        ]),
        source_link: String::from("https://github.com/TheRustyPickle/Talon"),
    };
    let talon = Project {
        title_image: Some(String::from("/assets/talon_1.png")),
        name: String::from("Talon"),
        description: String::from(
            "A tool to generate on-demand data insights from public Telegram chats",
        ),
        badges: vec![
            "Rust".to_string(),
            "GUI".to_string(),
            "Telegram".to_string(),
            "Analytics".to_string(),
        ],
        content: talon_content,
    };

    let funnel_content = ProjectContent {
        content: ContentProject::Funnel,
        demo_link: Some(String::from("https://therustypickle.github.io/Funnel-Web/")),
        images: Some(vec![
            String::from("/assets/funnel_1.png"),
            String::from("/assets/funnel_2.png"),
            String::from("/assets/funnel_3.png"),
            String::from("/assets/funnel_4.png"),
        ]),
        source_link: String::from("https://github.com/TheRustyPickle/Funnel-Web"),
    };
    let funnel = Project {
        title_image: Some(String::from("/assets/funnel_1.png")),
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
        content: ContentProject::Chirp,
        demo_link: None,
        images: Some(vec![
            String::from("/assets/chirp_1.png"),
            String::from("/assets/chirp_2.png"),
        ]),
        source_link: String::from("https://github.com/TheRustyPickle/Chirp"),
    };
    let chirp = Project {
        title_image: Some(String::from("/assets/chirp_1.png")),
        name: String::from("Chirp"),
        description: String::from(
            "A chat app built from scratch using GTK4 and Rust with encryption",
        ),
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
        content: ContentProject::RepoDL,
        demo_link: Some(String::from("/repo")),
        images: Some(vec![String::from("/assets/github_dl.png")]),
        source_link: String::from("https://github.com/TheRustyPickle/My-Site"),
    };
    let repo_dl = Project {
        title_image: Some(String::from("/assets/github_dl.png")),
        name: String::from("Repo D/L"),
        description: String::from(
            "Straightforward web app to view GitHub Repo release download status",
        ),
        badges: vec![
            "Rust".to_string(),
            "Leptos".to_string(),
            "GitHub".to_string(),
            "Actix-Web".to_string(),
        ],
        content: repo_dl_content,
    };

    let reddit_dl_content = ProjectContent {
        content: ContentProject::RedditDL,
        demo_link: Some(String::from("/reddit")),
        images: Some(vec![String::from("/assets/dl_reddit.png")]),
        source_link: String::from("https://github.com/TheRustyPickle/My-Site"),
    };
    let reddit_dl = Project {
        title_image: Some(String::from("/assets/dl_reddit.png")),
        name: String::from("Reddit D/L"),
        description: String::from("A simple web app to download content from a reddit post"),
        badges: vec![
            "Rust".to_string(),
            "Leptos".to_string(),
            "Reddit".to_string(),
            "Actix-Web".to_string(),
        ],
        content: reddit_dl_content,
    };

    let selectable_table_content = ProjectContent {
        content: ContentProject::Table,
        demo_link: Some(String::from(
            "https://therustypickle.github.io/egui-selectable-table/",
        )),
        images: Some(vec![String::from("/assets/table.png")]),
        source_link: String::from("https://github.com/TheRustyPickle/egui-selectable-table"),
    };
    let selectable_table = Project {
        title_image: Some(String::from("/assets/table.png")),
        name: String::from("egui Selectable Table"),
        description: String::from(
            "A library for egui to create tables with draggable cell and row selection.",
        ),
        badges: vec![
            "Rust".to_string(),
            "egui".to_string(),
            "Library".to_string(),
            "Widget".to_string(),
        ],
        content: selectable_table_content,
    };

    let theme_lerp_content = ProjectContent {
        content: ContentProject::Theme,
        demo_link: Some(String::from(
            "https://therustypickle.github.io/egui-theme-lerp/",
        )),
        images: None,
        source_link: String::from("https://github.com/TheRustyPickle/egui-theme-lerp"),
    };
    let theme_lerp = Project {
        title_image: Some(String::from("/assets/placeholder.svg")),
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
        content: ContentProject::Pulse,
        demo_link: None,
        images: None,
        source_link: String::from("https://github.com/TheRustyPickle/Pulse"),
    };
    let pulse = Project {
        title_image: Some(String::from("/assets/placeholder.svg")),
        name: String::from("Pulse"),
        description: String::from(
            "A Discord bot for scheduling messages with simple configuration",
        ),
        badges: vec!["Rust".to_string(), "Discord".to_string(), "Bot".to_string()],
        content: pulse_content,
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
    ]
}

fn get_project_content(project: ContentProject) -> impl IntoView {
    match project {
        ContentProject::Rex => view! {
            <p class="text-lg font-medium">
                "Rex is a terminal user interface app for managing incomes, expenses, and transactions.
                Built with Rust and Ratatui, it features a simple interface that’s easy to use."
            </p>

            <p class="mt-4 text-lg font-semibold">"Key features include:"</p>

            <ul class="mt-2 list-disc list-inside space-y-2">
                <li>"Easily view, add, edit, and delete transactions."</li>
                <li>
                    "Navigate through transactions and instantly observe balance changes 
                    after each transaction."
                </li>
                <li>
                    "Chart visualization of balance changes over a specific month, year, or all transactions."
                </li>
                <li>
                    "Access a summary with key insights on income, expense, and percentage distribution."
                </li>
                <li>"Built using SQLite, keeping everything local."</li>
                <li>Find transactions quickly using partial or specific information.</li>
                <li>"Organize transactions with custom tags for easy filtering."</li>
                <li>Works fully offline.</li>
            </ul>
        }.into_any(),
        ContentProject::Talon => view! {
            <p class="text-lg font-medium">
                "Talon is a tool to generate on-demand data insights from public Telegram chats. Powered by Rust, grammers, and egui, it offers a straightforward interface that leverages the Telegram account API."
            </p>

            <p class="mt-4 text-lg font-semibold">"Features:"</p>

            <ul class="mt-2 list-dict list-inside space-y-2">
                <li>
                    <strong>"User and Message Metrics:"</strong>
                    " Displays the number of unique users, total messages counted, and other info."
                </li>
                <li>
                    <strong>"Detailed User Insights:"</strong>
                    " View comprehensive user details including name, username, ID, total messages, total words, total characters, and more."
                </li>
                <li>
                    <strong>"Interactive Data Table:"</strong>
                    " Select cells, interact with the table, and copy data in an organized manner."
                </li>
                <li>
                    <strong>"Visual Analytics:"</strong>
                    " Visualize message counts and active users on an hourly, daily, weekly, monthly, and day-of-the-week basis."
                </li>
                <li>
                    <strong>"Date Range and Navigation:"</strong>
                    " Easily navigate and view table and chart data within a specific date range with buttons to cycle by day, week, month, or year."
                </li>
                <li>
                    <strong>"Session Management:"</strong>
                    " Choose between temporary sessions (logs out on app close) or non-temporary sessions (creates a file for persistent login)."
                </li>
                <li>
                    <strong>"User Grouping:"</strong>
                    " Group specific users by whitelisting to analyze their activity separately."
                </li>
                <li>
                    <strong>"Blacklisting:"</strong>
                    " Exclude specific users from data analysis to prevent their data from appearing in the results."
                </li>
                <li>
                    <strong>"Multi-Session Capability:"</strong>
                    " Utilize multiple sessions to dramatically increase checking speed, tested with up to 12 sessions and 300k messages."
                </li>
                <li>
                    <strong>"Multi-Chat Capability:"</strong>
                    " Analyze multiple chats simultaneously and view data from each chat separately."
                </li>
            </ul>
        }.into_any(),
        ContentProject::Funnel => view! {
            <p class="text-lg font-medium">
                "Funnel is a platform for visualizing Discord analytics, built with Rust and egui with WASM compatibility"
            </p>

            <p class="mt-4 text-lg font-semibold">"Primary Components:"</p>

            <ul class="mt-2 list-disc list-inside space-y-2">
                <li>
                    <strong>"Overview:"</strong>
                    " Summarizes key metrics such as total messages, unique users, and the most active channels and users. Includes a chart tracking member movement (e.g., joins and leaves)."
                </li>
                <li>
                    <strong>"User Table:"</strong>
                    " Displays all users along with message counts, word usage, and other activity details."
                </li>
                <li>
                    <strong>"Channel Table:"</strong>
                    " Provides message statistics for each channel, allowing easy comparison of activity levels."
                </li>
                <li>
                    <strong>"Message Chart:"</strong>
                    " Visualizes total and deleted messages over time. Supports adding specific users for detailed analysis across daily, hourly, weekly, and monthly intervals."
                </li>
                <li>
                    <strong>"User Activity Chart:"</strong>
                    " Tracks active user counts over different timeframes, helping to identify engagement trends."
                </li>
                <li>
                    <strong>"Common Words Analysis:"</strong>
                    " Highlights the most frequently used words or phrases in messages to reveal discussion trends."
                </li>
                <li>
                    <strong>"Channel Filter:"</strong>
                    " Enables filtering all analytics by selected channels, allowing focused analysis."
                </li>
            </ul>
        }.into_any(),
        ContentProject::Chirp => view! {
            <p class="text-lg font-medium">
                "Chirp is a chat application built from scratch using GTK4 in Rust, offering a native Linux experience with a strong focus on security and real-time communication."
            </p>

            <p class="mt-4 text-lg font-semibold">"Core Capabilities:"</p>

            <ul class="mt-2 list-disc list-inside space-y-2">
                <li>
                    <strong>"🎨 User Interface:"</strong>
                    " Designed with GTK4-rs for a smooth, native Linux experience."
                </li>
                <li>
                    <strong>"🌐 WebSocket Server:"</strong>
                    " Built with actix-web, enabling multi-client support with automatic reconnection."
                </li>
                <li>
                    <strong>"🛡️ Security:"</strong>
                    " Implements TLS-encrypted server communication and token-based authentication for secure access."
                </li>
                <li>
                    <strong>"💬 Messaging:"</strong>
                    " Supports sending and deleting messages, creating new chats, and synchronizing messages on startup."
                </li>
                <li>
                    <strong>"🔒 Message Encryption:"</strong>
                    " Uses a hybrid RSA + AES encryption system to protect messages, ensuring they are decrypted locally for display."
                </li>
            </ul>
        }.into_any(),
        ContentProject::RepoDL => view! {
            <p class="text-lg font-medium">
                "A simple tool for tracking GitHub repository releases and download statistics, providing quick insights into release popularity and overall download trends."
            </p>

            <p class="mt-4 text-lg font-semibold">"Insights Provided:"</p>

            <ul class="mt-2 list-disc list-inside space-y-2">
                <li>
                    <strong>"📊 Total Downloads:"</strong>
                    " Displays the cumulative download count across all releases."
                </li>
                <li>
                    <strong>"🏆 Most Popular Release:"</strong>
                    " Identifies the release with the highest number of downloads."
                </li>
                <li>
                    <strong>"📅 Release Breakdown:"</strong>
                    " Shows download statistics for each release, including total downloads and per-file download counts."
                </li>
            </ul>
        }.into_any(),
        ContentProject::RedditDL => view! {
            <p class="text-lg font-medium">
                "A lightweight tool for quickly downloading videos and images from Reddit posts"
            </p>
        }.into_any(),
        ContentProject::Table => view! {
            <p class="text-lg font-medium">
                "A library for egui that enables creating tables with draggable cell and row selection, offering flexibility and performance for large datasets."
            </p>

            <p class="mt-4 text-lg font-semibold">"Key Capabilities:"</p>

            <ul class="mt-2 list-disc list-inside space-y-2">
                <li>
                    <strong>"Draggable Selection:"</strong>
                    " Supports both individual cell and full-row selection while dragging."
                </li>
                <li>
                    <strong>"Automatic Scrolling:"</strong>
                    " Enables vertical table scrolling during drag, with adjustable sensitivity."
                </li>
                <li>
                    <strong>"Sortable Headers:"</strong>
                    " Allows sorting rows by clicking headers, with both ascending and descending order."
                </li>
                <li>
                    <strong>"Customizable UI:"</strong>
                    " Provides flexibility to modify row and header appearance."
                </li>
                <li>
                    <strong>"Keyboard Shortcuts:"</strong>
                    " Includes built-in support for 'Select All' (Ctrl+A) and 'Copy' (Ctrl+C)."
                </li>
                <li>
                    <strong>"High Performance:"</strong>
                    " Efficiently handles large datasets (1M+ rows) with proper configuration."
                </li>
            </ul>
        }.into_any(),
        ContentProject::Theme => view! {
            <p class="text-lg font-medium">
                "A lightweight library for egui that enables smooth theme transitions by linearly interpolating between any two visuals or themes."
            </p>

            <p class="mt-4 text-lg font-semibold">"Core Functionality:"</p>

            <ul class="mt-2 list-disc list-inside space-y-2">
                <li>
                    <strong>"Seamless Transitions:"</strong>
                    " Interpolates between two egui themes for smooth visual changes."
                </li>
                <li>
                    <strong>"Adjustable Timing:"</strong>
                    " Allows customization of transition speed for a tailored experience."
                </li>
                <li>
                    <strong>"Lightweight and Efficient:"</strong>
                    " Designed to be minimal, with no unnecessary overhead."
                </li>
            </ul>
        }.into_any(),
        ContentProject::Pulse => view! {
            <p class="text-lg font-medium">
                "Pulse is a straightforward Discord bot for scheduling messages with customization options. Built with Rust and the Serenity library, it prioritizes ease of use with a simple configuration system."
            </p>

            <p class="mt-4 text-lg font-semibold">"Core Capabilities:"</p>

            <ul class="mt-2 list-disc list-inside space-y-2">
                <li>
                    <strong>"Scheduled Messaging:"</strong>
                    " Automate message delivery in Discord servers with precise timing."
                </li>
                <li>
                    <strong>"Versatile Message Support:"</strong>
                    " Send text, file attachments, polls, and quizzes effortlessly."
                </li>
                <li>
                    <strong>"Robust Error Handling:"</strong>
                    " Prevents crashes in most cases, ensuring stable operation."
                </li>
                <li>
                    <strong>"Simple Configuration:"</strong>
                    " Uses easy-to-read JSON files with no complex nesting."
                </li>
                <li>
                    <strong>"Configuration Reloading:"</strong>
                    " No bot restarts required when modifying schedules—only bot settings need a restart."
                </li>
            </ul>
        }.into_any(),
    }
}
