use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 21a8 8 0 0 1 13.292-6" ></ path > < circle cy = "8" r = "5" cx = "10" ></ circle > < path d = "m16 19 2 2 4-4" ></ path > < / > } } pub const LUCIDE_USER_ROUND_CHECK : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;