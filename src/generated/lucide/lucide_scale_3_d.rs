use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "19" cy = "19" r = "2" ></ circle > < circle r = "2" cy = "5" cx = "5" ></ circle > < path d = "M5 7v12h12" ></ path > < path d = "m5 19 6-6" ></ path > < / > } } pub const LUCIDE_SCALE_3_D : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24")] } ;