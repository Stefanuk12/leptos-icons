use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "7" r = "3" ></ circle > < path d = "M10 9v6" ></ path > < circle r = "3" cx = "17" cy = "12" ></ circle > < path d = "M14 7v8" ></ path > < / > } } pub const LUCIDE_CASE_LOWER : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;