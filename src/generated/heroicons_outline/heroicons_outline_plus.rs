use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 4.5v15m7.5-7.5h-15" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_PLUS : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;