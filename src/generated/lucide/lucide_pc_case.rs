use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" rx = "2" height = "20" y = "2" x = "5" ></ rect > < path d = "M15 14h.01" ></ path > < path d = "M9 6h6" ></ path > < path d = "M9 10h6" ></ path > < / > } } pub const LUCIDE_PC_CASE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;