use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "18" height = "18" x = "3" rx = "2" ></ rect > < path d = "M7 7h.01" ></ path > < path d = "M17 7h.01" ></ path > < path d = "M7 17h.01" ></ path > < path d = "M17 17h.01" ></ path > < / > } } pub const LUCIDE_INSPECTION_PANEL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;