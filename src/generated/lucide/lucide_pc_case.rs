use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" height = "20" width = "14" x = "5" rx = "2" ></ rect > < path d = "M15 14h.01" ></ path > < path d = "M9 6h6" ></ path > < path d = "M9 10h6" ></ path > < / > } } pub const LUCIDE_PC_CASE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;