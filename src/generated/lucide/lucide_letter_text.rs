use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 12h6" ></ path > < path d = "M15 6h6" ></ path > < path d = "m3 13 3.553-7.724a.5.5 0 0 1 .894 0L11 13" ></ path > < path d = "M3 18h18" ></ path > < path d = "M4 11h6" ></ path > < / > } } pub const LUCIDE_LETTER_TEXT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;