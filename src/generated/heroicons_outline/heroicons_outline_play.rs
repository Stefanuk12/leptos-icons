use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_PLAY : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon")] } ;