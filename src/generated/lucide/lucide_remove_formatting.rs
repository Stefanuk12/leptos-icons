use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 7V4h16v3" ></ path > < path d = "M5 20h6" ></ path > < path d = "M13 4 8 20" ></ path > < path d = "m15 15 5 5" ></ path > < path d = "m20 15-5 5" ></ path > < / > } } pub const LUCIDE_REMOVE_FORMATTING : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;