use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" rx = "2" x = "14" width = "4" y = "14" ></ rect > < rect y = "4" height = "6" rx = "2" x = "6" width = "4" ></ rect > < path d = "M6 20h4" ></ path > < path d = "M14 10h4" ></ path > < path d = "M6 14h2v6" ></ path > < path d = "M14 4h2v6" ></ path > < / > } } pub const LUCIDE_BINARY : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none")] } ;