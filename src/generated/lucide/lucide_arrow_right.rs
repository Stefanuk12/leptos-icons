use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 12h14" ></ path > < path d = "m12 5 7 7-7 7" ></ path > < / > } } pub const LUCIDE_ARROW_RIGHT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round")] } ;