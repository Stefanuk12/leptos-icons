use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "12" cx = "7" ></ circle > < path d = "M10 9v6" ></ path > < circle r = "3" cy = "12" cx = "17" ></ circle > < path d = "M14 7v8" ></ path > < / > } } pub const LUCIDE_CASE_LOWER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor")] } ;