use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 10 2.5-2.5L3 5" ></ path > < path d = "m3 19 2.5-2.5L3 14" ></ path > < path d = "M10 6h11" ></ path > < path d = "M10 12h11" ></ path > < path d = "M10 18h11" ></ path > < / > } } pub const LUCIDE_LIST_COLLAPSE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24")] } ;