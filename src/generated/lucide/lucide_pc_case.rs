use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "5" width = "14" height = "20" y = "2" rx = "2" ></ rect > < path d = "M15 14h.01" ></ path > < path d = "M9 6h6" ></ path > < path d = "M9 10h6" ></ path > < / > } } pub const LUCIDE_PC_CASE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;