use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 12h8" ></ path > < path d = "M4 18V6" ></ path > < path d = "M12 18V6" ></ path > < path d = "m17 12 3-2v8" ></ path > < / > } } pub const LUCIDE_HEADING_1 : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;