use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" width = "20" x = "2" y = "6" height = "8" ></ rect > < path d = "M17 14v7" ></ path > < path d = "M7 14v7" ></ path > < path d = "M17 3v3" ></ path > < path d = "M7 3v3" ></ path > < path d = "M10 14 2.3 6.3" ></ path > < path d = "m14 6 7.7 7.7" ></ path > < path d = "m8 6 8 8" ></ path > < / > } } pub const LUCIDE_CONSTRUCTION : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;