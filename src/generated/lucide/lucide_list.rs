use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 12h.01" ></ path > < path d = "M3 18h.01" ></ path > < path d = "M3 6h.01" ></ path > < path d = "M8 12h13" ></ path > < path d = "M8 18h13" ></ path > < path d = "M8 6h13" ></ path > < / > } } pub const LUCIDE_LIST : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;