use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.5 13h6" ></ path > < path d = "m2 16 4.5-9 4.5 9" ></ path > < path d = "M18 7v9" ></ path > < path d = "m14 12 4 4 4-4" ></ path > < / > } } pub const LUCIDE_A_ARROW_DOWN : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;