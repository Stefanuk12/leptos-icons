use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 17 2 2 4-4" ></ path > < path d = "m3 7 2 2 4-4" ></ path > < path d = "M13 6h8" ></ path > < path d = "M13 12h8" ></ path > < path d = "M13 18h8" ></ path > < / > } } pub const LUCIDE_LIST_CHECKS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;