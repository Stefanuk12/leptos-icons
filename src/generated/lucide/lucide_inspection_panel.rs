use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" rx = "2" width = "18" height = "18" ></ rect > < path d = "M7 7h.01" ></ path > < path d = "M17 7h.01" ></ path > < path d = "M7 17h.01" ></ path > < path d = "M17 17h.01" ></ path > < / > } } pub const LUCIDE_INSPECTION_PANEL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;