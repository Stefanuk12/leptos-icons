use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.6 2.7a10 10 0 1 0 5.7 5.7" ></ path > < circle cy = "12" cx = "12" r = "2" ></ circle > < path d = "M13.4 10.6 19 5" ></ path > < / > } } pub const LUCIDE_CIRCLE_GAUGE : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;