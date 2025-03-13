use api::reddit_downloader;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::Title;
use shared::extract_reddit_id;
use shared::models::{DlType, DownloadData, DownloadMetadata, Downloads};
use thaw::{Button, ButtonAppearance, ButtonShape, Card, Icon, Input, InputPrefix, InputSize};

use crate::utils::create_blob_url;

#[component]
pub fn RedditDL() -> impl IntoView {
    let link = RwSignal::new(String::from(""));
    let (loading, set_loading) = signal(false);
    let (downloadables, set_downloadables) = signal(None);
    let (error_resp, set_error) = signal(String::new());

    let fetch_downloads = move |post_id| {
        set_downloadables.set(None);
        set_error.set(String::new());
        set_loading.set(true);
        spawn_local(async move {
            let result = reddit_downloader(post_id).await;

            match result {
                Ok(downloadables) => set_downloadables.set(Some(downloadables)),
                Err(e) => set_error.set(e.to_string()),
            };
            set_loading.set(false);
        });
    };

    let valid_reddit_link = move |_| {
        if let Some(post_id) = extract_reddit_id(&link.get()) {
            fetch_downloads(post_id.to_string())
        } else {
            set_error.set(String::from("No valid reddit link was found."));
        }
    };

    let download_button_text = move || {
        if loading.get() {
            view! {
                <div class="flex justify-center item-center gap-2">
                    <span>"Loading..."</span>
                </div>
            }
            .into_any()
        } else {
            view! { "Get Contents" }.into_any()
        }
    };

    view! {
        <Title text="Reddit D/L | Rusty Pickle" />

        <div class="flex items-center justify-center px-2">
            <Card class="!gap-0 w-11/12 sm:w-4/5 m-10 md:w-3/5 lg:w-1/2 xl:w-2/5 mb-20 !rounded-lg">
                <h4 class="text-xl font-semibold text-gray-700 mb-2 flex item-center justify-center">
                    "Reddit Post Downloader"
                </h4>
                <Input
                    class="w-full p-2"
                    placeholder="https://reddit.com/r/subreddit/comments/1234/post-details/"
                    value=link
                    size=InputSize::Medium
                >
                    <InputPrefix slot>
                        <Icon icon=icondata::AiRedditOutlined />
                    </InputPrefix>
                </Input>

                <Button
                    appearance=ButtonAppearance::Primary
                    shape=ButtonShape::Circular
                    class="mt-2 w-full !text-white font-semibold"
                    on_click=valid_reddit_link
                    loading
                    disabled=loading
                >
                    {move || download_button_text()}
                </Button>

                <Show
                    when=move || !error_resp.get().is_empty()
                    fallback=move || {
                        if downloadables.get().is_some() {
                            view! { <ShowDownloadables data=downloadables /> }.into_any()
                        } else {
                            view! { "" }.into_any()
                        }
                    }
                >
                    <p class="text-red-500 mt-2">{error_resp.get()}</p>
                </Show>
            </Card>
        </div>
    }
}

#[component]
fn show_downloadables(data: ReadSignal<Option<Downloads>>) -> impl IntoView {
    let blob_urls = Memo::new(move |_| {
        data.get().map(|downloads| {
            downloads
                .data
                .iter()
                .map(|item| {
                    let mime_type = format!("video/{}", item.metadata.extension);
                    let blob_url = create_blob_url(mime_type, &item.content);
                    (item.metadata.clone(), blob_url)
                })
                .collect::<Vec<_>>()
        })
    });

    let render_video = move || {
        if let Some(urls) = blob_urls.get() {
            if !urls.is_empty() {
                let filename = format!("File: {}.{}", urls[0].0.file_name, urls[0].0.extension);
                view! {
                    <div class="w-full max-w-md mx-auto">
                        <video class="w-full max-h-60 rounded-md" controls>
                            <source src=urls[0].1.clone() type="video/mp4" />
                        </video>
                        <p class="mt-2 text-gray-700 text-sm text-center">{filename}</p>
                    </div>
                }
                .into_any()
            } else {
                view! { "" }.into_any()
            }
        } else {
            view! { "" }.into_any()
        }
    };

    view! {
        <Show
            when=move || data.get().is_some() && !data.get().unwrap().data.is_empty()
            fallback=move || view! { "" }.into_any()
        >
            {move || match data.get().unwrap().download_type {
                DlType::Image => {
                    view! {
                        <div class="flex flex-col gap-2 w-full mt-4">
                            <For
                                each=move || data.get().unwrap().data
                                key=|item| item.metadata.file_name.clone()
                                children=move |item| {
                                    view! { <RenderImage item=item /> }
                                }
                            />
                        </div>
                    }
                        .into_any()
                }
                DlType::Video => {
                    let urls_for_buttons = blob_urls.get();
                    view! {
                        <div class="flex flex-col gap-2 w-full mt-4">
                            {render_video()} <div class="flex flex-row gap-2 w-full">
                                <For
                                    each=move || urls_for_buttons.clone().unwrap()
                                    key=|(item, _)| item.file_name.clone()
                                    children=move |(item, url)| {
                                        view! { <VideoDownloadButton item url=url.to_string() /> }
                                    }
                                />
                            </div>
                        </div>
                    }
                        .into_any()
                }
            }}
        </Show>
    }
}

#[component]
fn render_image(item: DownloadData) -> impl IntoView {
    let mime_type = format!("image/{}", item.metadata.extension);

    let url = create_blob_url(mime_type, &item.content);

    let filename = format!(
        "File: {}.{}",
        item.metadata.file_name, item.metadata.extension
    );

    view! {
        <img
            class="w-full h-auto max-h-60 object-cover rounded-md"
            src=url.clone()
            alt="Downloadable image"
        />

        <p class="mt-2 text-gray-700 text-sm text-center">{filename.clone()}</p>

        <a class="py-2 flex items-center justify-center" href=url download=filename>
            <Button appearance=ButtonAppearance::Primary shape=ButtonShape::Circular class="w-full">
                "Download"
            </Button>
        </a>
    }
}

#[component]
fn video_download_button(item: DownloadMetadata, url: String) -> impl IntoView {
    let filename = format!("{}.{}", item.file_name, item.extension);
    let resolution = format!("{}x{}", item.sizing.width, item.sizing.height);
    view! {
        <a class="w-full py-2 flex items-center justify-center" href=url download=filename>
            <Button appearance=ButtonAppearance::Primary shape=ButtonShape::Circular class="w-full">
                {resolution}
            </Button>
        </a>
    }
}
