use std::collections::HashMap;

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
    pub props: &'static [(&'static str, &'static str)],
}

macro_rules! attr_if_exists {
    ($props:expr, $svg:expr, $attr:expr, $value:expr) => {
        if let Some(x) = $value.get() {
            $props.remove(stringify!($attr));
            $svg = $svg.attr(stringify!($attr), x);
        }
    };
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
        let mut props = path
            .props
            .iter()
            .map(|x| x.to_owned())
            .collect::<HashMap<_, _>>();

        attr_if_exists!(props, svg, class, class);
        attr_if_exists!(props, svg, style, style);
        attr_if_exists!(props, svg, width, width);
        attr_if_exists!(props, svg, height, height);
        attr_if_exists!(props, svg, fill, fill);
        attr_if_exists!(props, svg, stroke_width, stroke_width);
        attr_if_exists!(props, svg, stroke, stroke);

        for (k, v) in props {
            svg = svg.attr(k, v);
        }

        svg.child((path.path)())
    };

    svg.into_view()
}
