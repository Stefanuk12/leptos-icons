use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m16 3 4 4-4 4" ></ path > < path d = "M20 7H4" ></ path > < path d = "m8 21-4-4 4-4" ></ path > < path d = "M4 17h16" ></ path > < / > } } pub const LUCIDE_ARROW_RIGHT_LEFT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;