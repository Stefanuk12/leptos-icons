use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 10.01h.01" ></ path > < path d = "M10 14.01h.01" ></ path > < path d = "M14 10.01h.01" ></ path > < path d = "M14 14.01h.01" ></ path > < path d = "M18 6v11.5" ></ path > < path d = "M6 6v12" ></ path > < rect x = "2" width = "20" y = "6" height = "12" rx = "2" ></ rect > < / > } } pub const LUCIDE_BANDAGE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;