use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "m4.5 15.75 7.5-7.5 7.5 7.5" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_UP : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("data-slot" , "icon") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;