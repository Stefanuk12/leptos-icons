use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" rx = "2" width = "4" x = "14" y = "14" ></ rect > < rect width = "4" height = "6" x = "6" rx = "2" y = "4" ></ rect > < path d = "M6 20h4" ></ path > < path d = "M14 10h4" ></ path > < path d = "M6 14h2v6" ></ path > < path d = "M14 4h2v6" ></ path > < / > } } pub const LUCIDE_BINARY : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;