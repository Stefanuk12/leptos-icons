use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m15.5 7.5 2.3 2.3a1 1 0 0 0 1.4 0l2.1-2.1a1 1 0 0 0 0-1.4L19 4" ></ path > < path d = "m21 2-9.6 9.6" ></ path > < circle cx = "7.5" cy = "15.5" r = "5.5" ></ circle > < / > } } pub const LUCIDE_KEY : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;