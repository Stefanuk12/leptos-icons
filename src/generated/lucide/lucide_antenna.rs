use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 12 7 2" ></ path > < path d = "m7 12 5-10" ></ path > < path d = "m12 12 5-10" ></ path > < path d = "m17 12 5-10" ></ path > < path d = "M4.5 7h15" ></ path > < path d = "M12 16v6" ></ path > < / > } } pub const LUCIDE_ANTENNA : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;