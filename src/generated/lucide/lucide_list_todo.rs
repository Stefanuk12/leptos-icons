use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "1" height = "6" width = "6" y = "5" ></ rect > < path d = "m3 17 2 2 4-4" ></ path > < path d = "M13 6h8" ></ path > < path d = "M13 12h8" ></ path > < path d = "M13 18h8" ></ path > < / > } } pub const LUCIDE_LIST_TODO : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24")] } ;