use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "7" r = "3" cy = "12" ></ circle > < path d = "M10 9v6" ></ path > < circle cx = "17" r = "3" cy = "12" ></ circle > < path d = "M14 7v8" ></ path > < / > } } pub const LUCIDE_CASE_LOWER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;