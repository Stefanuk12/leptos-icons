use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 15 4-8 4 8" ></ path > < path d = "M4 13h6" ></ path > < circle cy = "12" r = "3" cx = "18" ></ circle > < path d = "M21 9v6" ></ path > < / > } } pub const LUCIDE_CASE_SENSITIVE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24")] } ;