use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 12H3" ></ path > < path d = "m11 18 6-6-6-6" ></ path > < path d = "M21 5v14" ></ path > < / > } } pub const LUCIDE_ARROW_RIGHT_TO_LINE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;