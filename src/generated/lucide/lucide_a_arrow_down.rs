use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.5 13h6" ></ path > < path d = "m2 16 4.5-9 4.5 9" ></ path > < path d = "M18 7v9" ></ path > < path d = "m14 12 4 4 4-4" ></ path > < / > } } pub const LUCIDE_A_ARROW_DOWN : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24")] } ;