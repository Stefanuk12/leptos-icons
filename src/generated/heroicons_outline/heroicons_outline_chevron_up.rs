use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "m4.5 15.75 7.5-7.5 7.5 7.5" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_UP : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;