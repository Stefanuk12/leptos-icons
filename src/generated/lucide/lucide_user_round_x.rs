use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 21a8 8 0 0 1 11.873-7" ></ path > < circle r = "5" cx = "10" cy = "8" ></ circle > < path d = "m17 17 5 5" ></ path > < path d = "m22 17-5 5" ></ path > < / > } } pub const LUCIDE_USER_ROUND_X : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;