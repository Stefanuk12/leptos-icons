use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.6 2.7a10 10 0 1 0 5.7 5.7" ></ path > < circle r = "2" cx = "12" cy = "12" ></ circle > < path d = "M13.4 10.6 19 5" ></ path > < / > } } pub const LUCIDE_CIRCLE_GAUGE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round")] } ;