use leptos::*;

mod generated;
pub use generated::*;

#[derive(Clone)]
pub struct Path {
    pub path: fn() -> leptos::Fragment,
    pub icon_type: IconType,
}

#[derive(Clone)]
pub enum IconType {
    HeroIcons(HeroIconsType),
    Lucide,
}

#[derive(Clone)]
pub enum HeroIconsType {
    Outline,
    Solid,
    Mini,
}

#[component]
pub fn Icon(
    path: Path,
    #[prop(into)]
    #[prop(optional)]
    class: MaybeSignal<String>,

    #[prop(into)]
    #[prop(optional)]
    fill: MaybeSignal<String>,

    #[prop(into)]
    #[prop(optional)]
    stroke_width: MaybeSignal<String>,

    #[prop(into)]
    #[prop(optional)]
    stroke: MaybeSignal<String>,

    #[prop(into)]
    #[prop(optional)]
    width: MaybeSignal<String>,

    #[prop(into)]
    #[prop(optional)]
    height: MaybeSignal<String>,
) -> impl IntoView {
    match path.icon_type {
        IconType::HeroIcons(HeroType) => match HeroType {
            HeroIconsType::Outline => {
                view! {cx,
                    <svg xmlns="http://www.w3.org/2000/svg" class=class.get() viewBox="0 0 24 24"
                        data-license="From https://github.com/tailwindlabs/heroicons - Licensed under MIT"
                        width=move || width.get().is_empty().then_some("24".to_string()).unwrap_or(width.get())
                        height=move || height.get().is_empty().then_some("24".to_string()).unwrap_or(height.get())
                        fill=move || fill.get().is_empty().then_some("none".to_string()).unwrap_or(fill.get())
                        stroke-width=move || stroke_width.get().is_empty().then_some("2".to_string()).unwrap_or(stroke_width.get())
                        stroke=move || stroke.get().is_empty().then_some("currentColor".to_string()).unwrap_or(stroke.get())
                    >
                        {(path.path)()}
                    </svg>
                }
            }
            HeroIconsType::Solid => {
                view! {cx,
                    <svg xmlns="http://www.w3.org/2000/svg" class=class.get() viewBox="0 0 24 24"
                        data-license="From https://github.com/tailwindlabs/heroicons - Licensed under MIT"
                        width=move || width.get().is_empty().then_some("24".to_string()).unwrap_or(width.get())
                        height=move || height.get().is_empty().then_some("24".to_string()).unwrap_or(height.get())
                        fill=move || fill.get().is_empty().then_some("currentColor".to_string()).unwrap_or(fill.get())
                        stroke-width=move || stroke_width.get().is_empty().then_some("2".to_string()).unwrap_or(stroke_width.get()) stroke=move || stroke.get().is_empty().then_some("currentColor".to_string()).unwrap_or(stroke.get())
                    >
                        {(path.path)()}
                    </svg>
                }
            }
            HeroIconsType::Mini => {
                view! {cx,
                    <svg xmlns="http://www.w3.org/2000/svg" class=class.get() viewBox="0 0 20 20"
                        data-license="From https://github.com/tailwindlabs/heroicons - Licensed under MIT"
                        width=move || width.get().is_empty().then_some("20".to_string()).unwrap_or(width.get())
                        height=move || height.get().is_empty().then_some("20".to_string()).unwrap_or(height.get())
                        fill=move || fill.get().is_empty().then_some("currentColor".to_string()).unwrap_or(fill.get())
                        stroke-width=move || stroke_width.get().is_empty().then_some("2".to_string()).unwrap_or(stroke_width.get()) stroke=move || stroke.get().is_empty().then_some("currentColor".to_string()).unwrap_or(stroke.get())
                    >
                        {(path.path)()}
                    </svg>

                }
            }
        },
        IconType::Lucide => {
            view! {cx,
                <svg xmlns="http://www.w3.org/2000/svg" class=class.get() viewBox="0 0 24 24"
                    data-license="From https://github.com/lucide-icons/lucide - Licensed under ISC"
                        width=move || width.get().is_empty().then_some("24".to_string()).unwrap_or(width.get())
                        height=move || height.get().is_empty().then_some("24".to_string()).unwrap_or(height.get())
                    fill=move || fill.get().is_empty().then_some("none".to_string()).unwrap_or(fill.get())
                    stroke-width=move || stroke_width.get().is_empty().then_some("2".to_string()).unwrap_or(stroke_width.get()) stroke=move || stroke.get().is_empty().then_some("currentColor".to_string()).unwrap_or(stroke.get()) 
                    stroke-linecap="round" stroke-linejoin="round"
                >
                    {(path.path)()}
                </svg>
            }
        }
    }
}
