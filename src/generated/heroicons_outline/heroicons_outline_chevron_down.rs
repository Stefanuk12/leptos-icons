use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "m19.5 8.25-7.5 7.5-7.5-7.5" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_DOWN : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;