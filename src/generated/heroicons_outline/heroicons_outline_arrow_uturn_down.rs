use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m15 15-6 6m0 0-6-6m6 6V9a6 6 0 0 1 12 0v3" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UTURN_DOWN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("data-slot" , "icon") , ("fill" , "none")] } ;