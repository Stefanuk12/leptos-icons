use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "16" cy = "4" r = "1" ></ circle > < path d = "m18 19 1-7-6 1" ></ path > < path d = "m5 8 3-3 5.5 3-2.36 3.5" ></ path > < path d = "M4.24 14.5a5 5 0 0 0 6.88 6" ></ path > < path d = "M13.76 17.5a5 5 0 0 0-6.88-6" ></ path > < / > } } pub const LUCIDE_ACCESSIBILITY : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;