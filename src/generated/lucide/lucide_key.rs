use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "15.5" cx = "7.5" r = "5.5" ></ circle > < path d = "m21 2-9.6 9.6" ></ path > < path d = "m15.5 7.5 3 3L22 7l-3-3" ></ path > < / > } } pub const LUCIDE_KEY : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor")] } ;