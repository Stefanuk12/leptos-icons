use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M4.5 10.5 12 3m0 0 7.5 7.5M12 3v18" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UP : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("fill" , "none") , ("data-slot" , "icon")] } ;