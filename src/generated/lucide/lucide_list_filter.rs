use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 6h18" ></ path > < path d = "M7 12h10" ></ path > < path d = "M10 18h4" ></ path > < / > } } pub const LUCIDE_LIST_FILTER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;