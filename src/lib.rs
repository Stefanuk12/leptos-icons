use leptos::*;

mod generated;
pub use generated::*;

#[macro_export]
macro_rules! maybe_text_prop_some {
    ($e:expr) => {
        ::leptos::MaybeProp::derive(move || Some(::leptos::TextProp::from($e)))
    };
}

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

/// `MaybeProp` might be cumbersome to use, but there is a helper macro [`maybe_text_prop_some!`] that can be used to simplify the usage.
/// 
/// Icon
#[component]
pub fn Icon(
    path: Path,

    #[prop(into)]
    #[prop(optional)]
    class: MaybeProp<leptos::TextProp>,

    #[prop(into)]
    #[prop(optional)]
    style: MaybeProp<leptos::TextProp>,

    #[prop(into)]
    #[prop(optional)]
    fill: MaybeProp<leptos::TextProp>,

    #[prop(into)]
    #[prop(optional)]
    stroke_width: MaybeProp<leptos::TextProp>,

    #[prop(into)]
    #[prop(optional)]
    stroke: MaybeProp<leptos::TextProp>,

    #[prop(into)]
    #[prop(optional)]
    width: MaybeProp<leptos::TextProp>,

    #[prop(into)]
    #[prop(optional)]
    height: MaybeProp<leptos::TextProp>,
) -> impl IntoView {

    let svg = move || {
        let mut svg = leptos::svg::svg();

        svg = svg.attr("xmlns", "http://www.w3.org/2000/svg");

        if let Some(x) = class.get() {
            svg = svg.attr("class", x);
        }

        if let Some(x) = style.get() {
            svg = svg.attr("style", x);
        }

        if let Some(x) = width.get() {
            svg = svg.attr("width", x);
        }

        if let Some(x) = height.get() {
            svg = svg.attr("height", x);
        }

        if let Some(x) = fill.get() {
            svg = svg.attr("fill", x);
        }

        if let Some(x) = stroke_width.get() {
            svg = svg.attr("stroke-width", x);
        }

        if let Some(x) = stroke.get() {
            svg = svg.attr("stroke", x);
        }

        match &path.icon_type {
            IconType::HeroIcons(HeroType) => match HeroType {
                HeroIconsType::Outline => {
                    svg = svg.attr("data-license", "From https://github.com/tailwindlabs/heroicons - Licensed under MIT");
                    svg = svg.attr("viewBox", "0 0 24 24");
                },
                HeroIconsType::Solid => {
                    svg = svg.attr("data-license", "From https://github.com/tailwindlabs/heroicons - Licensed under MIT");
                    svg = svg.attr("viewBox", "0 0 24 24");
                },
                HeroIconsType::Mini => {
                    svg = svg.attr("data-license", "From https://github.com/tailwindlabs/heroicons - Licensed under MIT");
                    svg = svg.attr("viewBox", "0 0 20 20");
                }
            },
            IconType::Lucide => {
                svg = svg.attr("data-license", "From https://github.com/lucide-icons/lucide - Licensed under ISC");
                svg = svg.attr("viewBox", "0 0 24 24");
            }
        }

        svg.child((path.path)())
    };

    svg.into_view()
}
