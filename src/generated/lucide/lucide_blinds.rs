use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3h18" ></ path > < path d = "M20 7H8" ></ path > < path d = "M20 11H8" ></ path > < path d = "M10 19h10" ></ path > < path d = "M8 15h12" ></ path > < path d = "M4 3v14" ></ path > < circle r = "2" cx = "4" cy = "19" ></ circle > < / > } } pub const LUCIDE_BLINDS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24")] } ;