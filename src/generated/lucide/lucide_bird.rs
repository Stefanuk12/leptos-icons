use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 7h.01" ></ path > < path d = "M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 20" ></ path > < path d = "m20 7 2 .5-2 .5" ></ path > < path d = "M10 18v3" ></ path > < path d = "M14 17.75V21" ></ path > < path d = "M7 18a6 6 0 0 0 3.84-10.61" ></ path > < / > } } pub const LUCIDE_BIRD : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;