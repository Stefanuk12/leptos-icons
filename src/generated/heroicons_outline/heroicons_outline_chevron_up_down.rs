use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "M8.25 15 12 18.75 15.75 15m-7.5-6L12 5.25 15.75 9" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHEVRON_UP_DOWN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("stroke-width" , "1.5") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;