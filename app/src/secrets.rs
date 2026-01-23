use anyhow::{Context, Result as AResult};
use api::get_secret;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use leptos_workers::worker;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thaw::{
    Button, ButtonAppearance, ButtonShape, Card, Icon, Input, InputPrefix, InputSize, Radio,
    RadioGroup, Spinner,
};
use vial_core::crypto::{decrypt_with_password, decrypt_with_random_key};
use vial_shared::{EncryptedPayload, FullSecretV1, Payload};

#[derive(Serialize, Deserialize, Clone)]
struct WorkerRequest {
    payload: EncryptedPayload,
    hash: String,
    key: String,
    schema: String,
}

#[worker(MyFutureWorker)]
fn my_worker(request: WorkerRequest) -> Result<FullSecretV1, String> {
    let WorkerRequest {
        payload,
        hash,
        key,
        schema,
    } = &request;

    let result = if hash.is_empty() {
        match schema.as_str() {
            "Password" => decrypt_password(key, &payload.payload),
            "Random" => decrypt_random_key(key, &payload.payload),
            _ => unreachable!(),
        }
    } else {
        decrypt_random_key(hash, &payload.payload)
    };

    result.map_err(|e| format!("{e:#}"))
}

#[component]
pub fn Secrets() -> impl IntoView {
    let params = use_params_map();

    let id = move || params.read().get("id").unwrap_or_default();

    let (payload, set_payload) = signal(None::<EncryptedPayload>);
    let (failed_payload, set_failed_payload) = signal((false, String::new()));
    let loading = RwSignal::new(false);

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
    let radio_value = RwSignal::new(String::from("Password"));

    let (pending, set_pending) = signal(None::<Arc<FullSecretV1>>);

    let (decrypted_secret, set_decrypted_secret) = signal(None::<Arc<FullSecretV1>>);
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
        let _ = radio_value.get();
        set_error.set(String::new());
    });

    let create_error = move |e: String| {
        set_error.set(format!("Failed to decrypt secret. Error: {e:#}"));
        loading.set(false);
        set_decrypt_key.set(String::new());
    };

    Effect::new(move |_| {
        let pending = pending.get();

        if let Some(pending) = pending {
            set_decrypted_secret.set(Some(pending));
        }
    });

    let initiate_decrypt = move || {
        let payload = payload.get();
        let key = decrypt_key.get();
        let hash = hash.get();

        if decrypted_secret.get().is_some() {
            create_error(String::from("Secret has already been decrypted"));
            return;
        }

        let Some(payload) = payload else {
            create_error(String::from("Payload not found"));
            return;
        };

        if key.is_empty() {
            create_error(String::from("Decrypt key cannot be empty"));
            return;
        }

        let request = WorkerRequest {
            payload,
            hash,
            key: decrypt_key.get(),
            schema: radio_value.get(),
        };

        spawn_local(async move {
            let decrypt_result = my_worker(request).await;

            if let Err(e) = decrypt_result {
                create_error(e.to_string());
                return;
            }

            let decrypt_result = decrypt_result.unwrap();

            if let Err(e) = decrypt_result {
                create_error(e.to_string());
                return;
            }

            let secret = decrypt_result.unwrap();

            loading.set(false);
            set_pending.set(Some(Arc::new(secret)));
        });
    };

    let submit_response = move || {
        let inputted_key = inputted_key.get();

        if inputted_key.is_empty() {
            set_error.set(String::from("Please enter a key"));
            return;
        };

        loading.set(true);
        set_decrypt_key.set(inputted_key);
        set_error.set(String::new());
        initiate_decrypt();
    };

    let suspense_fallback = move || {
        view! {
            <div>
                <Spinner />
            </div>
        }
    };

    let payload_error = move || {
        let (_, error) = failed_payload.get();
        view! {
            <p class="text-red-500 dark:text-red-400">
                {format!("Failed to fetch secret. Error: {error}")}
            </p>
        }
    };

    let get_key_from_user = move || {
        view! {
            <div class="flex flex-col gap-2">
                <Input
                    class="w-full p-2"
                    placeholder="Secret key or the password used to encrypt this secret"
                    value=inputted_key
                    size=InputSize::Large
                    on:keypress=move |e| {
                        if e.char_code() == 13 {
                            submit_response();
                        }
                    }
                >
                    <InputPrefix slot>
                        <Icon icon=icondata::FaRedditBrands />
                    </InputPrefix>
                </Input>

                <Show when=move || !decrypt_error.get().is_empty()>
                    <p class="text-red-500 dark:text-red-400">{move || decrypt_error.get()}</p>
                </Show>

                <RadioGroup value=radio_value>
                    <Radio value="Password" label="Use password schema" />
                    <Radio value="Random" label="Use random key schema" />
                </RadioGroup>

                <Button
                    appearance=ButtonAppearance::Primary
                    shape=ButtonShape::Circular
                    class="mt-2 w-full text-white! dark:text-gray-100! font-semibold"
                    on_click=move |_| submit_response()
                    loading
                >
                    <Show when=move || loading.get() fallback=|| view! { "Submit" }>
                        <div class="flex justify-center item-center gap-2">
                            <span>"Loading..."</span>
                        </div>
                    </Show>
                </Button>
            </div>
        }
    };

    let show_payload_result = move || view! { <p>"Payload found alongside hash"</p> }.into_any();

    view! {
        <div class="flex justify-center items-center">
            <Suspense fallback=move || suspense_fallback>

                <div>
                    <Card class="rounded-lg!">
                        <Show
                            when=move || {
                                let (status, _) = failed_payload.get();
                                !status
                            }
                            fallback=move || payload_error
                        >
                            <Show
                                when=move || {
                                    hash.get().is_empty() && decrypted_secret.get().is_none()
                                }
                                fallback=move || show_payload_result
                            >
                                <div>
                                    {move || get_key_from_user}
                                </div>
                            </Show>
                        </Show>

                    </Card>
                </div>
                {move || secret_payload.get().map(handle_payload)}
            </Suspense>
        </div>

    }
}

fn decrypt_random_key(key: &str, payload: &[u8]) -> AResult<FullSecretV1> {
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

fn decrypt_password(key: &str, payload: &[u8]) -> AResult<FullSecretV1> {
    let decrypted = decrypt_with_password(payload, key).context("Failed to decrypt secret")?;

    let full_secret = Payload::from_bytes(decrypted)
        .context("Failed to serialize secret")?
        .to_full_secret()
        .context("Failed to serialize secret")?;

    Ok(full_secret)
}
