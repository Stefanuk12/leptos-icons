use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "4.5" r = "2.5" ></ circle > < path d = "m10.2 6.3-3.9 3.9" ></ path > < circle cy = "12" cx = "4.5" r = "2.5" ></ circle > < path d = "M7 12h10" ></ path > < circle r = "2.5" cx = "19.5" cy = "12" ></ circle > < path d = "m13.8 17.7 3.9-3.9" ></ path > < circle r = "2.5" cy = "19.5" cx = "12" ></ circle > < / > } } pub const LUCIDE_WAYPOINTS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none")] } ;