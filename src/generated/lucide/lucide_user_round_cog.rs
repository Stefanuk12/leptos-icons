use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 21a8 8 0 0 1 10.434-7.62" ></ path > < circle r = "5" cx = "10" cy = "8" ></ circle > < circle cx = "18" cy = "18" r = "3" ></ circle > < path d = "m19.5 14.3-.4.9" ></ path > < path d = "m16.9 20.8-.4.9" ></ path > < path d = "m21.7 19.5-.9-.4" ></ path > < path d = "m15.2 16.9-.9-.4" ></ path > < path d = "m21.7 16.5-.9.4" ></ path > < path d = "m15.2 19.1-.9.4" ></ path > < path d = "m19.5 21.7-.4-.9" ></ path > < path d = "m16.9 15.2-.4-.9" ></ path > < / > } } pub const LUCIDE_USER_ROUND_COG : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;