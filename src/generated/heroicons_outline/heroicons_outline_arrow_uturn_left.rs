use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M9 15 3 9m0 0 6-6M3 9h12a6 6 0 0 1 0 12h-3" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UTURN_LEFT : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24")] } ;