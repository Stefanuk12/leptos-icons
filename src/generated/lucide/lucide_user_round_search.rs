use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "8" r = "5" cx = "10" ></ circle > < path d = "M2 21a8 8 0 0 1 10.434-7.62" ></ path > < circle cy = "18" r = "3" cx = "18" ></ circle > < path d = "m22 22-1.9-1.9" ></ path > < / > } } pub const LUCIDE_USER_ROUND_SEARCH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;