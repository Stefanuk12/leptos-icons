use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m4.5 18.75 7.5-7.5 7.5 7.5" ></ path > < path stroke - linejoin = "round" stroke - linecap = "round" d = "m4.5 12.75 7.5-7.5 7.5 7.5" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_DOUBLE_UP : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("stroke" , "currentColor") , ("data-slot" , "icon")] } ;