use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "2" cy = "5" cx = "19" ></ circle > < circle r = "2" cx = "5" cy = "19" ></ circle > < path d = "M5 17A12 12 0 0 1 17 5" ></ path > < / > } } pub const LUCIDE_SPLINE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;