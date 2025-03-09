use api::reddit_downloader;
use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::{extract_reddit_id, DlType, DownloadData, DownloadMetadata, Downloads};
use std::collections::HashMap;
use thaw::{
    Button, ButtonAppearance, ButtonShape, Card, ConfigProvider, Icon, Input, InputPrefix, Theme,
};

use crate::utils::create_blob_url;

#[component]
pub fn HomePage() -> impl IntoView {
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

    let link = RwSignal::new(String::from(""));
    let (loading, set_loading) = signal(false);
    let (downloadables, set_downloadables) = signal(None);
    let (error_resp, set_error) = signal(String::new());

    let fetch_downloads = move |_| {
        let link_to_use = link.get().clone();
        set_downloadables.set(None);
        set_error.set(String::new());
        set_loading.set(true);
        spawn_local(async move {
            let result = reddit_downloader(link_to_use).await;

            match result {
                Ok(downloadables) => set_downloadables.set(Some(downloadables)),
                Err(e) => set_error.set(e.to_string()),
            };
            set_loading.set(false);
        });
    };

    let valid_reddit_link = move |m| {
        if extract_reddit_id(&link.get()).is_some() {
            fetch_downloads(m)
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
        <ConfigProvider theme>
            <div class="flex items-center justify-center min-h-screen bg-gray-100">
                <Card class="!gap-0 p-6 w-11/12 sm:w-4/5 md:w-3/5 lg:w-1/2 xl:w-2/5">
                    <h4 class="text-xl font-semibold text-gray-700 mb-2 flex item-center justify-center">
                        "Reddit Downloader"
                    </h4>
                    <Input
                        class="w-full p-2"
                        placeholder="https://reddit.com/r/subreddit/comments/1234/post-details/"
                        value=link
                    >
                        <InputPrefix slot>
                            <Icon icon=icondata::AiLinkOutlined />
                        </InputPrefix>
                    </Input>

                    <Button
                        appearance=ButtonAppearance::Primary
                        shape=ButtonShape::Circular
                        class="mt-2 w-full !text-white font-semibold"
                        on_click=valid_reddit_link
                        loading
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
        </ConfigProvider>
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
                view! {
                    <div class="w-full max-w-md mx-auto">
                        <video class="w-full max-h-60 rounded-md" controls>
                            <source src=urls[0].1.clone() type="video/mp4" />
                        </video>
                        <p class="mt-2 text-gray-700 text-sm text-center">
                            File: {urls[0].0.file_name.clone()}.{urls[0].0.extension.clone()}
                        </p>
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

    let filename = format!("{}.{}", item.metadata.file_name, item.metadata.extension);

    view! {
        <img
            class="w-full h-auto max-h-60 object-cover rounded-md"
            src=url.clone()
            alt="Downloadable image"
        />

        <p class="mt-2 text-gray-700 text-sm text-center">File: {filename.clone()}</p>

        <a
            class="w-full bg-n-blue hover:bg-h-blue rounded-full text-white font-semibold py-2 transition duration-300 flex items-center justify-center"
            href=url
            download=filename
        >
            "Download"
        </a>
    }
}

#[component]
fn video_download_button(item: DownloadMetadata, url: String) -> impl IntoView {
    let filename = format!("{}.{}", item.file_name, item.extension);
    let resolution = format!("{}x{}", item.sizing.width, item.sizing.height);
    view! {
        <a
            class="w-full bg-n-blue hover:bg-h-blue text-white font-semibold py-2 rounded-full transition duration-300 flex items-center justify-center"
            href=url
            download=filename
        >
            {resolution}
        </a>
    }
}
