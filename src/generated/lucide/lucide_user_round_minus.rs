use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 21a8 8 0 0 1 13.292-6" ></ path > < circle cy = "8" cx = "10" r = "5" ></ circle > < path d = "M22 19h-6" ></ path > < / > } } pub const LUCIDE_USER_ROUND_MINUS : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor")] } ;