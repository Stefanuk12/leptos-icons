use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 16 4 4 4-4" ></ path > < path d = "M7 20V4" ></ path > < path d = "m21 8-4-4-4 4" ></ path > < path d = "M17 4v16" ></ path > < / > } } pub const LUCIDE_ARROW_DOWN_UP : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24")] } ;