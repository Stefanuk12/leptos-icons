use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "3" x = "3" width = "18" height = "18" ></ rect > < path d = "M7 7h.01" ></ path > < path d = "M17 7h.01" ></ path > < path d = "M7 17h.01" ></ path > < path d = "M17 17h.01" ></ path > < / > } } pub const LUCIDE_INSPECTION_PANEL : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;