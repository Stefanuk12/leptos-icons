use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "M15.75 19.5 8.25 12l7.5-7.5" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_LEFT : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "none") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5")] } ;