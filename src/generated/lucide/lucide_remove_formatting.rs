use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 7V4h16v3" ></ path > < path d = "M5 20h6" ></ path > < path d = "M13 4 8 20" ></ path > < path d = "m15 15 5 5" ></ path > < path d = "m20 15-5 5" ></ path > < / > } } pub const LUCIDE_REMOVE_FORMATTING : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;