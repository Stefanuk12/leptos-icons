use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "7" cy = "7" r = "5" ></ circle > < circle cx = "17" cy = "17" r = "5" ></ circle > < path d = "M12 17h10" ></ path > < path d = "m3.46 10.54 7.08-7.08" ></ path > < / > } } pub const LUCIDE_TABLETS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none")] } ;