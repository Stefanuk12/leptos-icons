use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 7 9 3 5 7l4 4" ></ path > < path d = "m17 11 4 4-4 4-4-4" ></ path > < path d = "m8 12 4 4 6-6-4-4Z" ></ path > < path d = "m16 8 3-3" ></ path > < path d = "M9 21a6 6 0 0 0-6-6" ></ path > < / > } } pub const LUCIDE_SATELLITE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;