use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m15 15 6-6m0 0-6-6m6 6H9a6 6 0 0 0 0 12h3" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UTURN_RIGHT : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;