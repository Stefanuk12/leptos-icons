use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "7" r = "5" cx = "7" ></ circle > < circle r = "5" cx = "17" cy = "17" ></ circle > < path d = "M12 17h10" ></ path > < path d = "m3.46 10.54 7.08-7.08" ></ path > < / > } } pub const LUCIDE_TABLETS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;