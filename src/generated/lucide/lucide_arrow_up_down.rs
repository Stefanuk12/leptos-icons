use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m21 16-4 4-4-4" ></ path > < path d = "M17 20V4" ></ path > < path d = "m3 8 4-4 4 4" ></ path > < path d = "M7 4v16" ></ path > < / > } } pub const LUCIDE_ARROW_UP_DOWN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;