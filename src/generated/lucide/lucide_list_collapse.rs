use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 10 2.5-2.5L3 5" ></ path > < path d = "m3 19 2.5-2.5L3 14" ></ path > < path d = "M10 6h11" ></ path > < path d = "M10 12h11" ></ path > < path d = "M10 18h11" ></ path > < / > } } pub const LUCIDE_LIST_COLLAPSE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;