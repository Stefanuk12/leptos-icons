use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 10 2.5-2.5L3 5" ></ path > < path d = "m3 19 2.5-2.5L3 14" ></ path > < path d = "M10 6h11" ></ path > < path d = "M10 12h11" ></ path > < path d = "M10 18h11" ></ path > < / > } } pub const LUCIDE_LIST_COLLAPSE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;