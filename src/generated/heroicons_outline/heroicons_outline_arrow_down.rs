use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19.5 13.5 12 21m0 0-7.5-7.5M12 21V3" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_DOWN : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("aria-hidden" , "true")] } ;