use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M15.75 17.25 12 21m0 0-3.75-3.75M12 21V3" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_LONG_DOWN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;