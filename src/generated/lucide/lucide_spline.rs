use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "19" cy = "5" r = "2" ></ circle > < circle r = "2" cy = "19" cx = "5" ></ circle > < path d = "M5 17A12 12 0 0 1 17 5" ></ path > < / > } } pub const LUCIDE_SPLINE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;