use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M12 4.5v15m7.5-7.5h-15" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_PLUS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("data-slot" , "icon") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "1.5")] } ;