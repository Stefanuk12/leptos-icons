use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "4.5" r = "2.5" ></ circle > < path d = "m10.2 6.3-3.9 3.9" ></ path > < circle cy = "12" cx = "4.5" r = "2.5" ></ circle > < path d = "M7 12h10" ></ path > < circle cy = "12" r = "2.5" cx = "19.5" ></ circle > < path d = "m13.8 17.7 3.9-3.9" ></ path > < circle r = "2.5" cx = "12" cy = "19.5" ></ circle > < / > } } pub const LUCIDE_WAYPOINTS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;