use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M3 4.5h14.25M3 9h9.75M3 13.5h9.75m4.5-4.5v12m0 0-3.75-3.75M17.25 21 21 17.25" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_BARS_ARROW_DOWN : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;