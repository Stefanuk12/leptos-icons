use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3h18" ></ path > < path d = "M20 7H8" ></ path > < path d = "M20 11H8" ></ path > < path d = "M10 19h10" ></ path > < path d = "M8 15h12" ></ path > < path d = "M4 3v14" ></ path > < circle cy = "19" cx = "4" r = "2" ></ circle > < / > } } pub const LUCIDE_BLINDS : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;