use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 4.5h14.25M3 9h9.75M3 13.5h9.75m4.5-4.5v12m0 0-3.75-3.75M17.25 21 21 17.25" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_BARS_ARROW_DOWN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("data-slot" , "icon")] } ;