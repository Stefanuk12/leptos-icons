use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < path d = "M6 12c0-1.7.7-3.2 1.8-4.2" ></ path > < circle cy = "12" cx = "12" r = "2" ></ circle > < path d = "M18 12c0 1.7-.7 3.2-1.8 4.2" ></ path > < / > } } pub const LUCIDE_DISC_3 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;