use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 12h8" ></ path > < path d = "M4 18V6" ></ path > < path d = "M12 18V6" ></ path > < path d = "M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2" ></ path > < path d = "M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2" ></ path > < / > } } pub const LUCIDE_HEADING_3 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;