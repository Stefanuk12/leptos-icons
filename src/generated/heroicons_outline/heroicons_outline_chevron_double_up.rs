use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4.5 18.75 7.5-7.5 7.5 7.5" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < path stroke - linejoin = "round" d = "m4.5 12.75 7.5-7.5 7.5 7.5" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_DOUBLE_UP : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("data-slot" , "icon")] } ;