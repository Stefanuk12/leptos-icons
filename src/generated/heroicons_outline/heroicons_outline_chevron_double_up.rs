use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4.5 18.75 7.5-7.5 7.5 7.5" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < path stroke - linejoin = "round" d = "m4.5 12.75 7.5-7.5 7.5 7.5" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_DOUBLE_UP : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;