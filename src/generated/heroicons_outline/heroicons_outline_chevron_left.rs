use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M15.75 19.5 8.25 12l7.5-7.5" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_LEFT : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("fill" , "none") , ("aria-hidden" , "true")] } ;