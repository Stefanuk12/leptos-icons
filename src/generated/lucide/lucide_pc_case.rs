use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" rx = "2" x = "5" height = "20" width = "14" ></ rect > < path d = "M15 14h.01" ></ path > < path d = "M9 6h6" ></ path > < path d = "M9 10h6" ></ path > < / > } } pub const LUCIDE_PC_CASE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor")] } ;