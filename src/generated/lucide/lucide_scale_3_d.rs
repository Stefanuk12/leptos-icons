use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "19" r = "2" cx = "19" ></ circle > < circle cx = "5" r = "2" cy = "5" ></ circle > < path d = "M5 7v12h12" ></ path > < path d = "m5 19 6-6" ></ path > < / > } } pub const LUCIDE_SCALE_3_D : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;