use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" rx = "1" y = "6" height = "8" width = "20" ></ rect > < path d = "M17 14v7" ></ path > < path d = "M7 14v7" ></ path > < path d = "M17 3v3" ></ path > < path d = "M7 3v3" ></ path > < path d = "M10 14 2.3 6.3" ></ path > < path d = "m14 6 7.7 7.7" ></ path > < path d = "m8 6 8 8" ></ path > < / > } } pub const LUCIDE_CONSTRUCTION : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;