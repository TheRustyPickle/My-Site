use leptos::prelude::*;
use thaw::Button;

#[component]
pub fn test_page() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <Carousel />
        </div>
    }
}

#[component]
fn Carousel() -> impl IntoView {
    // Sample images - replace with your actual images
    let images = vec![
        "https://fastly.picsum.photos/id/2/5000/3333.jpg?hmac=_KDkqQVttXw_nM-RyJfLImIbafFrqLsuGO5YuHqD-qQ",
        "https://fastly.picsum.photos/id/1/5000/3333.jpg?hmac=Asv2DU3rA_5D1xSe22xZK47WEAN0wjWeFOhzd13ujW4",
        "https://fastly.picsum.photos/id/0/5000/3333.jpg?hmac=_j6ghY5fCfSD6tvtcV74zXivkJSPIfR9B8w34XeQmvU"
    ];
    // State for current image index
    let (current_index, set_current_index) = signal(0);

    // State for next image index during animation
    let (next_index, set_next_index) = signal(0);

    // State for tracking if animation is in progress
    let (is_animating, set_is_animating) = signal(false);

    // State for tracking the slide direction (for animation)
    let (slide_direction, set_slide_direction) = signal("none");

    // Total number of images
    let total_images = images.len();

    // Reset carousel state after animation completes
    let reset_animation = move || {
        set_current_index.set(next_index.get());
        set_slide_direction.set("reset");

        // Small delay to ensure reset is applied after the animation classes are processed
        set_timeout(
            move || {
                set_slide_direction.set("none");
                set_is_animating.set(false);
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
        set_is_animating.set(true);

        // Complete the transition after animation finishes
        set_timeout(reset_animation, std::time::Duration::from_millis(500));
    };

    let next_no = move || {
        set_slide_direction.set("next");
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
        // First set to init-prev (instant positioning)
        set_slide_direction.set("init-prev");

        set_is_animating.set(true);

        // After a small delay, trigger the animation
        set_timeout(
            move || {
                set_slide_direction.set("prev");
            },
            std::time::Duration::from_millis(20),
        );

        // Complete the transition after animation finishes
        set_timeout(reset_animation, std::time::Duration::from_millis(520));
    };

    let prev_no = move || {
        set_slide_direction.set("init-prev");
        set_timeout(
            move || {
                set_slide_direction.set("prev");
            },
            std::time::Duration::from_millis(20),
        );

        // Complete the transition after animation finishes
        set_timeout(reset_animation, std::time::Duration::from_millis(520));
    };

    // Go to specific image
    let go_to_image = move |idx: usize| {
        if is_animating.get() || idx == current_index.get() {
            return;
        }

        set_next_index.set(idx);

        if idx > current_index.get() {
            next_no()
        } else {
            prev_no()
        }

        set_is_animating.set(true);

        // Complete the transition after animation finishes
        set_timeout(reset_animation, std::time::Duration::from_millis(500));
    };

    let carousel_button_class = move |idx: usize| {
        let mut initial = "transition-colors duration-200".to_string();

        let is_active = move || current_index.get() == idx;
        let is_animating = move || is_animating.get();
        if is_active() {
            initial.push_str(" !bg-blue-600");
        }

        if !is_active() {
            initial.push_str(" !bg-gray-300");
        }

        if is_animating() {
            initial.push_str(" !opacity-50");
        }

        initial
    };

    view! {
        <div class="flex">
            <div class="flex flex-col max-w-2xl mx-auto">
                // CSS for animations
                <style>
                    {".carousel-container {
                    width: 100%;
                    overflow: hidden;
                    position: relative;
                    height: 400px;
                    }
                    .carousel-track {
                    display: flex;
                    width: 200%;
                    height: 100%;
                    position: relative;
                    transition: transform 0.5s ease;
                    }
                    .carousel-slide {
                    width: 50%;
                    height: 100%;
                    flex-shrink: 0;
                    }
                    /* No animation state */
                    .slide-none .carousel-track {
                    transform: translateX(0);
                    }
                    /* Next slide animation - current slides left, next appears from right */
                    .slide-next .carousel-track {
                    transform: translateX(-50%);
                    }
                    /* Previous slide animation - current slides right, prev appears from left */
                    .slide-prev .carousel-track {
                    transform: translateX(0);
                    }
                    /* Init state for prev animation */
                    .init-prev .carousel-track {
                    transform: translateX(-50%);
                    transition: none;
                    }
                    /* Reset after animation completes */
                    .slide-reset .carousel-track {
                    transition: none;
                    transform: translateX(0);
                    }"}
                </style>

                <div class="relative w-full overflow-hidden rounded-lg shadow-lg">
                    // Carousel container with animation classes
                    <div
                        class="carousel-container"
                        class:slide-next=move || slide_direction.get() == "next"
                        class:slide-prev=move || slide_direction.get() == "prev"
                        class:slide-none=move || slide_direction.get() == "none"
                        class:slide-reset=move || slide_direction.get() == "reset"
                        class:init-prev=move || slide_direction.get() == "init-prev"
                    >
                        // Track that contains both slides
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
                                                    src=images[curr]
                                                    alt=format!("Image {}", curr + 1)
                                                    class="w-full h-full object-cover"
                                                />
                                            </div>
                                            // Next slide on the right
                                            <div class="carousel-slide">
                                                <img
                                                    src=images[next]
                                                    alt=format!("Image {}", next + 1)
                                                    class="w-full h-full object-cover"
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
                                                    src=images[next]
                                                    alt=format!("Image {}", next + 1)
                                                    class="w-full h-full object-cover"
                                                />
                                            </div>
                                            // Current slide on the right
                                            <div class="carousel-slide">
                                                <img
                                                    src=images[curr]
                                                    alt=format!("Image {}", curr + 1)
                                                    class="w-full h-full object-cover"
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
                                                    src=images[curr]
                                                    alt=format!("Image {}", curr + 1)
                                                    class="w-full h-full object-cover"
                                                />
                                            </div>
                                            <div class="carousel-slide"></div>
                                        }
                                            .into_any()
                                    }
                                }
                            }}
                        </div>
                    </div>

                    // Navigation arrows
                    <div class="absolute inset-0 flex items-center justify-between p-4 z-10">
                        <button
                            on:click=prev
                            class="bg-white bg-opacity-50 p-2 rounded-full shadow hover:bg-opacity-75 focus:outline-none transition-all duration-200"
                            class:opacity-50=move || is_animating.get()
                            class:cursor-not-allowed=move || is_animating.get()
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M15 19l-7-7 7-7"
                                />
                            </svg>
                        </button>

                        <button
                            on:click=next
                            class="bg-white bg-opacity-50 p-2 rounded-full shadow hover:bg-opacity-75 focus:outline-none transition-all duration-200"
                            class:opacity-50=move || is_animating.get()
                            class:cursor-not-allowed=move || is_animating.get()
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M9 5l7 7-7 7"
                                />
                            </svg>
                        </button>
                    </div>

                </div>

                // Image indicators

                <div class="flex mt-4 mb-4 flex-row justify-center gap-2">
                    {(0..total_images)
                        .map(|idx| {
                            view! {
                                <Button
                                    on:click=move |_| go_to_image(idx)
                                    class=carousel_button_class(idx)
                                >
                                    {idx + 1}
                                </Button>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
                // Counter display
                <div class="mt-2 text-sm text-gray-600">
                    {move || format!("Image {} of {}", current_index.get() + 1, total_images)}
                </div>
            </div>
        </div>
    }
}
