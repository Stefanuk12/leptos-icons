use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" rx = "2" x = "3" y = "3" ></ rect > < path d = "M7 7h.01" ></ path > < path d = "M17 7h.01" ></ path > < path d = "M7 17h.01" ></ path > < path d = "M17 17h.01" ></ path > < / > } } pub const LUCIDE_INSPECTION_PANEL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;