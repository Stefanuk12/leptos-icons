use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "3" cx = "7" ></ circle > < path d = "M10 9v6" ></ path > < circle cx = "17" r = "3" cy = "12" ></ circle > < path d = "M14 7v8" ></ path > < / > } } pub const LUCIDE_CASE_LOWER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;