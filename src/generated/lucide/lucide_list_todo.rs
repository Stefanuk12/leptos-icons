use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "5" rx = "1" x = "3" width = "6" height = "6" ></ rect > < path d = "m3 17 2 2 4-4" ></ path > < path d = "M13 6h8" ></ path > < path d = "M13 12h8" ></ path > < path d = "M13 18h8" ></ path > < / > } } pub const LUCIDE_LIST_TODO : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;