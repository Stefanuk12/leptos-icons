use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m15 15-6 6m0 0-6-6m6 6V9a6 6 0 0 1 12 0v3" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UTURN_DOWN : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;