use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "10" cy = "8" r = "5" ></ circle > < path d = "M2 21a8 8 0 0 1 10.434-7.62" ></ path > < circle r = "3" cx = "18" cy = "18" ></ circle > < path d = "m22 22-1.9-1.9" ></ path > < / > } } pub const LUCIDE_USER_ROUND_SEARCH : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;