use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M8.25 15 12 18.75 15.75 15m-7.5-6L12 5.25 15.75 9" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_UP_DOWN : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;