use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m8.25 4.5 7.5 7.5-7.5 7.5" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_RIGHT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("fill" , "none") , ("aria-hidden" , "true")] } ;