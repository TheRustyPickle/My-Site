use api::get_secret;
use leptos::prelude::ServerFnError;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use thaw::Spinner;
use vial_shared::EncryptedPayload;

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

    let show_payload_result = move || {
        let (status, error) = failed_payload.get();

        if status {
            view! { <p class="text-red-500 dark:text-red-400">{format!("Failed to fetch secret. Error {error}")}</p> }.into_any()
        } else if !status && payload.get().is_none() {
            view! {
                <div>
                    <Spinner />
                </div>
            }
            .into_any()
        } else {
            view! { <p>"Payload gotten"</p> }.into_any()
        }
    };

    view! {
        <div class="flex justify-center items-center">
            <Suspense fallback=move || show_payload_result>
                <div>
                    {move || { secret_payload.get().map(handle_payload) }}
                    {move || { show_payload_result }}
                </div>
            </Suspense>
        </div>
    }
}
