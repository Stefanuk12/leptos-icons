use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 12h.01" ></ path > < path d = "M3 18h.01" ></ path > < path d = "M3 6h.01" ></ path > < path d = "M8 12h13" ></ path > < path d = "M8 18h13" ></ path > < path d = "M8 6h13" ></ path > < / > } } pub const LUCIDE_LIST : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24")] } ;