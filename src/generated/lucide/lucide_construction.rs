use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "6" height = "8" rx = "1" width = "20" x = "2" ></ rect > < path d = "M17 14v7" ></ path > < path d = "M7 14v7" ></ path > < path d = "M17 3v3" ></ path > < path d = "M7 3v3" ></ path > < path d = "M10 14 2.3 6.3" ></ path > < path d = "m14 6 7.7 7.7" ></ path > < path d = "m8 6 8 8" ></ path > < / > } } pub const LUCIDE_CONSTRUCTION : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;