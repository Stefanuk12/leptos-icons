use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m18.75 4.5-7.5 7.5 7.5 7.5m-6-15L5.25 12l7.5 7.5" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_DOUBLE_LEFT : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;