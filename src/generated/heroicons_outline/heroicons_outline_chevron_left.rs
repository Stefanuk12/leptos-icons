use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.75 19.5 8.25 12l7.5-7.5" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_LEFT : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true")] } ;