use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16ZM7.346 5.294a.75.75 0 0 0-1.192.912L9.056 10H6.75a.75.75 0 0 0 0 1.5h2.5v1h-2.5a.75.75 0 0 0 0 1.5h2.5v1.25a.75.75 0 0 0 1.5 0V14h2.5a.75.75 0 1 0 0-1.5h-2.5v-1h2.5a.75.75 0 1 0 0-1.5h-2.306l2.902-3.794a.75.75 0 1 0-1.192-.912L10 8.765l-2.654-3.47Z" clip - rule = "evenodd" fill - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_CURRENCY_YEN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 20 20") , ("fill" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;