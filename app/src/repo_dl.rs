use api::github_checker;
use leptos::{prelude::*, task::spawn_local};
use leptos_meta::Title;
use shared::{
    extract_github_info,
    models::{ReleaseInfo, RepoReleasesSummary},
};
use thaw::{Button, ButtonAppearance, ButtonShape, Card, Icon, Input, InputPrefix, InputSize};

#[component]
pub fn RepoDL() -> impl IntoView {
    let link = RwSignal::new(String::new());
    let (loading, set_loading) = signal(false);
    let (error_resp, set_error) = signal(String::new());
    let (release_summary, set_release_summary) = signal(None);

    let content_button_text = move || {
        if loading.get() {
            view! {
                <div class="flex justify-center item-center gap-2">
                    <span>"Processing..."</span>
                </div>
            }
            .into_any()
        } else {
            view! { "Check Releases" }.into_any()
        }
    };

    let get_release_status = move |username, repo| {
        set_error.set(String::new());
        set_loading.set(true);
        spawn_local(async move {
            let result = github_checker(username, repo).await;

            match result {
                Ok(release) => {
                    set_release_summary.set(Some(release));
                }
                Err(e) => set_error.set(e.to_string()),
            };
            set_loading.set(false);
        });
    };

    let valid_github_link = move |_| {
        if let Some((username, repo)) = extract_github_info(&link.get()) {
            get_release_status(username, repo)
        } else {
            set_error.set(String::from("No valid reddit link was found."));
        }
    };

    view! {
        <Title text="GitHub D/L | Rusty Pickle" />

        <div class="flex items-center justify-center mb-2 px-2">
            <Card class="!gap-0 w-11/12 sm:w-4/5 md:w-3/5 lg:w-1/2 xl:w-2/5 !rounded-lg">
                <h4 class="text-xl font-semibold text-gray-700 mb-2 flex item-center justify-center">
                    "Github Release Status"
                </h4>

                <Input
                    class="w-full p-2"
                    placeholder="https://github.com/user/repository"
                    value=link
                    size=InputSize::Medium
                >
                    <InputPrefix slot>
                        <Icon icon=icondata::AiGithubOutlined />
                    </InputPrefix>
                </Input>

                <Button
                    appearance=ButtonAppearance::Primary
                    shape=ButtonShape::Circular
                    class="mt-2 w-full !text-white font-semibold"
                    on_click=valid_github_link
                    loading
                >
                    {move || content_button_text()}
                </Button>

                <Show
                    when=move || !error_resp.get().is_empty()
                    fallback=move || {
                        view! { "" }
                    }
                >
                    <p class="text-red-500 mt-2">{error_resp.get()}</p>
                </Show>
            </Card>

        </div>

        <Show
            when=move || !error_resp.get().is_empty()
            fallback=move || {
                if let Some(summary) = release_summary.get() {
                    view! { <ReleaseSummary release_summary=summary.clone() /> }.into_any()
                } else {
                    view! { "" }.into_any()
                }
            }
        >
            {}
        </Show>
    }
}

#[component]
fn ReleaseSummary(release_summary: RepoReleasesSummary) -> impl IntoView {
    let card_class = "!rounded-lg flex flex-col justify-center items-center h-30 hover:!bg-blue-50 hover:shadow-gray-400 hover:!shadow-xl !transition-all !duration-200 hover:scale-105 relative hover:z-10";

    let show_most_downloaded = move |release: Option<ReleaseInfo>| {
        if let Some(release) = release {
            view! {
                <a href=release.url>
                    <Card class=card_class>
                        <h3 class="text-lg font-semibold">"Most Downloaded Release"</h3>
                        <p class="text-xl font-bold">{release.tag.clone()}</p>
                        <p class="text-lg">"Downloads: " {release.total_downloads}</p>
                    </Card>
                </a>
            }
            .into_any()
        } else {
            view! {
                <Card class=card_class>
                    <h3 class="text-lg font-semibold">"Most Downloaded Release"</h3>
                    <p class="text-sm">"No data available"</p>
                </Card>
            }
            .into_any()
        }
    };

    let show_release_card = move |release: ReleaseInfo| {
        let card_class = "!rounded-lg hover:!bg-blue-50 hover:shadow-gray-400 hover:!shadow-xl !transition-all !duration-200 hover:scale-105 relative hover:z-10";
        let card = view! {
            <Card class=card_class>
                <h3 class="text-lg font-semibold">{release.tag.clone()}</h3>
                <p>"Total Downloads: " {release.total_downloads}</p>

                <ul class="mt-2 flex flex-col gap-1">
                    <For
                        each=move || release.assets.clone()
                        key=|asset| asset.name.clone()
                        children=move |asset| {
                            view! {
                                <li class="text-sm text-gray-700 flex justify-between">
                                    <span>{asset.name.clone()}</span>
                                    <span class="text-gray-500">
                                        {asset.download_count} " downloads"
                                    </span>
                                </li>
                            }
                        }
                    />
                </ul>
            </Card>
        }
        .into_any();

        view! {
            <div class="mt-2">
                <a href=release.url>{card}</a>
            </div>
        }
    };
    view! {
        <div class="w-full">
            <div class="px-2 flex flex-col justify-center gap-2 ">
                <Card class=card_class>
                    <h3 class="text-lg font-semibold">"Total Downloads"</h3>
                    <p class="text-2xl font-bold">{release_summary.total_downloads}</p>
                </Card>

                {show_most_downloaded(release_summary.most_downloaded_release.clone())}
            </div>

            <div class="px-2">
                <For
                    each=move || release_summary.releases.clone()
                    key=|release| release.tag.clone()
                    children=move |release| { show_release_card(release) }
                />
            </div>
        </div>
    }
}
