use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" x = "3" width = "6" rx = "1" y = "5" ></ rect > < path d = "m3 17 2 2 4-4" ></ path > < path d = "M13 6h8" ></ path > < path d = "M13 12h8" ></ path > < path d = "M13 18h8" ></ path > < / > } } pub const LUCIDE_LIST_TODO : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none")] } ;