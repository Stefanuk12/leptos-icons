use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "m4.5 5.25 7.5 7.5 7.5-7.5m-15 6 7.5 7.5 7.5-7.5" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_DOUBLE_DOWN : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("stroke-width" , "1.5")] } ;