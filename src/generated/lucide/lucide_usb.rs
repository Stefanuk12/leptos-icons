use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "10" cy = "7" r = "1" ></ circle > < circle cx = "4" cy = "20" r = "1" ></ circle > < path d = "M4.7 19.3 19 5" ></ path > < path d = "m21 3-3 1 2 2Z" ></ path > < path d = "M9.26 7.68 5 12l2 5" ></ path > < path d = "m10 14 5 2 3.5-3.5" ></ path > < path d = "m18 12 1-1 1 1-1 1Z" ></ path > < / > } } pub const LUCIDE_USB : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24")] } ;