use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "7" r = "5" cx = "7" ></ circle > < circle r = "5" cy = "17" cx = "17" ></ circle > < path d = "M12 17h10" ></ path > < path d = "m3.46 10.54 7.08-7.08" ></ path > < / > } } pub const LUCIDE_TABLETS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24")] } ;