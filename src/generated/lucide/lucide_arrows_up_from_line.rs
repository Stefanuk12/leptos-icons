use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4 6 3-3 3 3" ></ path > < path d = "M7 17V3" ></ path > < path d = "m14 6 3-3 3 3" ></ path > < path d = "M17 17V3" ></ path > < path d = "M4 21h16" ></ path > < / > } } pub const LUCIDE_ARROWS_UP_FROM_LINE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;