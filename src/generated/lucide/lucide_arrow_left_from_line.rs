use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m9 6-6 6 6 6" ></ path > < path d = "M3 12h14" ></ path > < path d = "M21 19V5" ></ path > < / > } } pub const LUCIDE_ARROW_LEFT_FROM_LINE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2")] } ;