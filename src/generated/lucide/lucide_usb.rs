use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "7" r = "1" cx = "10" ></ circle > < circle cx = "4" cy = "20" r = "1" ></ circle > < path d = "M4.7 19.3 19 5" ></ path > < path d = "m21 3-3 1 2 2Z" ></ path > < path d = "M9.26 7.68 5 12l2 5" ></ path > < path d = "m10 14 5 2 3.5-3.5" ></ path > < path d = "m18 12 1-1 1 1-1 1Z" ></ path > < / > } } pub const LUCIDE_USB : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;