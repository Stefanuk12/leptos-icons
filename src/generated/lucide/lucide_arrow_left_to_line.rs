use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 19V5" ></ path > < path d = "m13 6-6 6 6 6" ></ path > < path d = "M7 12h14" ></ path > < / > } } pub const LUCIDE_ARROW_LEFT_TO_LINE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;