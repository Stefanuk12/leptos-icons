use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12 19-7-7 7-7" ></ path > < path d = "M19 12H5" ></ path > < / > } } pub const LUCIDE_ARROW_LEFT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;