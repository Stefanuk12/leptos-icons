use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M12 4.5v15m7.5-7.5h-15" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_PLUS : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "none") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon")] } ;