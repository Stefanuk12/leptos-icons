use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4.5 15.75 7.5-7.5 7.5 7.5" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_UP : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("fill" , "none") , ("data-slot" , "icon")] } ;