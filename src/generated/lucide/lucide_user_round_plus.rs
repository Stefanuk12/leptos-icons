use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 21a8 8 0 0 1 13.292-6" ></ path > < circle r = "5" cx = "10" cy = "8" ></ circle > < path d = "M19 16v6" ></ path > < path d = "M22 19h-6" ></ path > < / > } } pub const LUCIDE_USER_ROUND_PLUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;