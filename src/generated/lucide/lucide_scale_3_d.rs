use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "19" cy = "19" r = "2" ></ circle > < circle r = "2" cx = "5" cy = "5" ></ circle > < path d = "M5 7v12h12" ></ path > < path d = "m5 19 6-6" ></ path > < / > } } pub const LUCIDE_SCALE_3_D : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;