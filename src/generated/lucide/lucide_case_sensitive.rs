use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 15 4-8 4 8" ></ path > < path d = "M4 13h6" ></ path > < circle r = "3" cy = "12" cx = "18" ></ circle > < path d = "M21 9v6" ></ path > < / > } } pub const LUCIDE_CASE_SENSITIVE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;