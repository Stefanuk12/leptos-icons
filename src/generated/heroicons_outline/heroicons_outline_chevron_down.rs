use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m19.5 8.25-7.5 7.5-7.5-7.5" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_DOWN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("fill" , "none") , ("aria-hidden" , "true")] } ;