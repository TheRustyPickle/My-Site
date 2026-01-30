use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{LazyRoute, hooks::use_navigate, lazy_route};
use thaw::{Button, ButtonAppearance, ButtonShape, Card, Icon, Input, InputPrefix, InputSize};

pub struct SecretIDView {}

#[lazy_route]
impl LazyRoute for SecretIDView {
    fn data() -> Self {
        Self {}
    }

    fn view(_this: Self) -> AnyView {
        let secrets_input = RwSignal::new(String::new());
        let (id_error, set_error) = signal(String::new());

        let (loading, set_loading) = signal(false);
        let (random_key, set_random_key) = signal(None::<String>);
        let (secret_id, set_secret_id) = signal(None::<String>);

        let (redirect, set_redirect) = signal(false);

        Effect::new(move |_| {
            let redirect = redirect.get();

            let secret_id = secret_id.get();
            let random_key = random_key.get();

            if redirect && let Some(id) = secret_id {
                let mut redirect_to = format!("/secrets/{}", id);

                if let Some(key) = random_key {
                    redirect_to = format!("{}#{}", redirect_to, key);
                };

                let navigate = use_navigate();
                navigate(&redirect_to, Default::default());
            }
        });

        let submit_response = move || {
            set_loading.set(true);

            let source = secrets_input.get();
            let Some(secret_id) = source.split('/').next_back() else {
                set_error.set("Could not find the secret id in the secret link.".to_string());
                set_loading.set(false);
                return;
            };

            let key = secret_id.split_once('#');

            if let Some((id, key)) = key {
                set_random_key.set(Some(key.to_string()));
                set_secret_id.set(Some(id.to_string()));
            } else {
                set_secret_id.set(Some(secret_id.to_string()));
            };

            set_redirect.set(true);
        };

        view! {
            <Title text="Secret | Rusty Pickle" />
            <div class="rounded-lg! w-full max-w-screen-sm mx-auto p-4 sm:p-6 flex flex-col gap-6">
                <Card class="rounded-lg!">
                    <div class="flex flex-col gap-4 w-full max-w-screen-sm mx-auto p-2 sm:p-4">
                        <Input
                            class="w-full p-3 text-sm sm:text-base"
                            placeholder="Enter secret ID or secret URL"
                            value=secrets_input
                            size=InputSize::Large
                            disabled=loading
                            on:keypress=move |e| {
                                if e.char_code() == 13 {
                                    submit_response();
                                }
                            }
                        >
                            <InputPrefix slot>
                                <Icon icon=icondata::FaKeySolid />
                            </InputPrefix>
                        </Input>
                        <Show when=move || !id_error.get().is_empty()>
                            <p class="text-red-500 dark:text-red-400 text-sm sm:text-base">
                                {move || id_error.get()}
                            </p>
                        </Show>
                        <Button
                            appearance=ButtonAppearance::Primary
                            shape=ButtonShape::Circular
                            class="mt-2 w-full sm:w-auto text-white! dark:text-gray-100! font-semibold"
                            on_click=move |_| submit_response()
                            loading
                        >
                            <Show when=move || loading.get() fallback=|| view! { "Submit" }>
                                <div class="flex justify-center items-center gap-2">
                                    <span>"Loading..."</span>
                                </div>
                            </Show>
                        </Button>

                    </div>

                </Card>

            </div>
        }
        .into_any()
    }
}
