use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "12" cx = "7" ></ circle > < path d = "M10 9v6" ></ path > < circle cx = "17" r = "3" cy = "12" ></ circle > < path d = "M14 7v8" ></ path > < / > } } pub const LUCIDE_CASE_LOWER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24")] } ;