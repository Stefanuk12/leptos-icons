use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "5" cx = "10" cy = "8" ></ circle > < path d = "M2 21a8 8 0 0 1 10.434-7.62" ></ path > < circle cx = "18" cy = "18" r = "3" ></ circle > < path d = "m22 22-1.9-1.9" ></ path > < / > } } pub const LUCIDE_USER_ROUND_SEARCH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;