use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 21a8 8 0 0 1 13.292-6" ></ path > < circle cx = "10" r = "5" cy = "8" ></ circle > < path d = "M19 16v6" ></ path > < path d = "M22 19h-6" ></ path > < / > } } pub const LUCIDE_USER_ROUND_PLUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;