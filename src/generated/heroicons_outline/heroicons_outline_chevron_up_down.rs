use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M8.25 15 12 18.75 15.75 15m-7.5-6L12 5.25 15.75 9" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_UP_DOWN : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("data-slot" , "icon") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;