use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "8" cx = "12" r = "5" ></ circle > < path d = "M20 21a8 8 0 0 0-16 0" ></ path > < / > } } pub const LUCIDE_USER_ROUND : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor")] } ;