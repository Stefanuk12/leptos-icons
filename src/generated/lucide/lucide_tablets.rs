use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "7" cy = "7" r = "5" ></ circle > < circle cy = "17" r = "5" cx = "17" ></ circle > < path d = "M12 17h10" ></ path > < path d = "m3.46 10.54 7.08-7.08" ></ path > < / > } } pub const LUCIDE_TABLETS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;