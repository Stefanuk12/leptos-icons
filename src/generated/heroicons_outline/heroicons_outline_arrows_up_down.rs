use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 7.5 7.5 3m0 0L12 7.5M7.5 3v13.5m13.5 0L16.5 21m0 0L12 16.5m4.5 4.5V7.5" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROWS_UP_DOWN : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "1.5") , ("stroke" , "currentColor")] } ;