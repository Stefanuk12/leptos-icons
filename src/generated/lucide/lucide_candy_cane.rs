use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.7 21a2 2 0 0 1-3.5-2l8.6-14a6 6 0 0 1 10.4 6 2 2 0 1 1-3.464-2 2 2 0 1 0-3.464-2Z" ></ path > < path d = "M17.75 7 15 2.1" ></ path > < path d = "M10.9 4.8 13 9" ></ path > < path d = "m7.9 9.7 2 4.4" ></ path > < path d = "M4.9 14.7 7 18.9" ></ path > < / > } } pub const LUCIDE_CANDY_CANE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;