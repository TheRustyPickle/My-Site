use anyhow::{Context, Result};
use api::get_secret;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use leptos::prelude::ServerFnError;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use thaw::{Card, Icon, Input, InputPrefix, InputSize, Spinner};
use vial_core::crypto::{decrypt_with_password, decrypt_with_random_key};
use vial_shared::{EncryptedPayload, FullSecretV1, Payload};

#[component]
pub fn Secrets() -> impl IntoView {
    let params = use_params_map();

    let id = move || params.read().get("id").unwrap_or_default();

    let (payload, set_payload) = signal(None::<EncryptedPayload>);
    let (failed_payload, set_failed_payload) = signal((false, String::new()));

    let secret_payload = Resource::new(id, move |id| async move { get_secret(id).await });

    let handle_payload = move |payload: Result<EncryptedPayload, ServerFnError>| match payload {
        Ok(payload) => {
            set_payload.set(Some(payload));
        }
        Err(e) => {
            set_failed_payload.set((true, e.to_string()));
        }
    };

    let (hash, set_hash) = signal(String::new());
    let (decrypt_key, set_decrypt_key) = signal(String::new());

    let inputted_key = RwSignal::new(String::new());

    let (decrypted_secret, set_decrypted_secret) = signal(None::<FullSecretV1>);
    let (decrypt_error, set_error) = signal(String::new());

    Effect::new(move |_| {
        let current_hash = window().location().hash().unwrap_or_default();
        let final_hash = current_hash
            .strip_prefix('#')
            .unwrap_or(&current_hash)
            .to_string();

        set_decrypt_key.set(final_hash.clone());
        set_hash.set(final_hash);
    });

    Effect::new(move |_| {
        let payload = payload.get();
        let key = decrypt_key.get();
        let hash = hash.get();

        if decrypted_secret.get().is_some() {
            return;
        }

        let Some(payload) = payload else {
            return;
        };

        if key.is_empty() {
            return;
        }

        let result = if hash.is_empty() {
            decrypt_password(&key, &payload.payload)
        } else {
            decrypt_random_key(&hash, &payload.payload)
        };

        match result {
            Ok(secret) => {
                log::info!("Decrypted secret");
                set_decrypted_secret.set(Some(secret));
            }
            Err(e) => {
                set_error.set(format!("Failed to decrypt secret. Error: {e}"));
            }
        }
    });

    let show_decryption_error = move || {
        let msg = decrypt_error.get();
        (!msg.is_empty()).then(|| {
            view! {
                <p class="text-red-500 dark:text-red-400">
                    {msg}
                </p>
            }
        })
    };

    let show_payload_result = move || {
        let (status, error) = failed_payload.get();

        if status {
            view! {
                <p class="text-red-500 dark:text-red-400">
                    {format!("Failed to fetch secret. Error: {error}")}
                </p>
            }
            .into_any()
        } else if !status && payload.get().is_none() {
            view! {
                <div>
                    <Spinner />
                </div>
            }
            .into_any()
        } else if hash.get().is_empty() {
            view! {
                <div>
                    <Input
                        class="w-full p-2"
                        placeholder="https://reddit.com/r/subreddit/comments/1234/post-details/"
                        value=inputted_key
                        size=InputSize::Large
                        on:keypress=move |e| {
                            if e.char_code() == 13 {
                                set_decrypt_key.set(inputted_key.get());
                            }
                        }
                    >
                        <InputPrefix slot>
                            <Icon icon=icondata::FaRedditBrands />
                        </InputPrefix>
                    </Input>

                    <Show
                        when=move || !decrypt_error.get().is_empty()
                    >
                        <p class="">
                            {move || show_decryption_error}
                        </p>
                    </Show>
                </div>
            }
            .into_any()
        } else {
            view! { <p>"Payload found alongside hash"</p> }.into_any()
        }
    };

    view! {
        <div class="flex justify-center items-center">
            <Suspense fallback=move || show_payload_result>

                <div>
                    <Card class="rounded-lg!">
                        {move || secret_payload.get().map(handle_payload)}
                        {move || show_payload_result}
                    </Card>
                </div>
            </Suspense>
        </div>
    }
}

fn decrypt_random_key(key: &str, payload: &[u8]) -> Result<FullSecretV1> {
    let decoded_key = URL_SAFE
        .decode(key)
        .context("Failed to decode key. Is the key valid?")?;

    let arr_ref: &[u8; 32] = decoded_key
        .as_slice()
        .try_into()
        .context("Failed to decode key. Is the key valid")?;

    let decrypted =
        decrypt_with_random_key(payload, arr_ref).context("Failed to decrypt secret")?;

    let full_secret = Payload::from_bytes(decrypted)
        .context("Failed to deserialize secret")?
        .to_full_secret()
        .context("Failed to deserialize secret")?;

    Ok(full_secret)
}

fn decrypt_password(key: &str, payload: &[u8]) -> Result<FullSecretV1> {
    let decrypted = decrypt_with_password(payload, key).context("Failed to decrypt secret")?;

    let full_secret = Payload::from_bytes(decrypted)
        .context("Failed to serialize secret")?
        .to_full_secret()
        .context("Failed to serialize secret")?;

    Ok(full_secret)
}
