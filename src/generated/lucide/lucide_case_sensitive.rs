use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 15 4-8 4 8" ></ path > < path d = "M4 13h6" ></ path > < circle cy = "12" r = "3" cx = "18" ></ circle > < path d = "M21 9v6" ></ path > < / > } } pub const LUCIDE_CASE_SENSITIVE : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;