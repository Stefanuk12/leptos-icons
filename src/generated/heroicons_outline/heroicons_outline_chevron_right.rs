use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "m8.25 4.5 7.5 7.5-7.5 7.5" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_RIGHT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon")] } ;