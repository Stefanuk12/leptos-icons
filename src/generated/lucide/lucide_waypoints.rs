use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "2.5" cx = "12" cy = "4.5" ></ circle > < path d = "m10.2 6.3-3.9 3.9" ></ path > < circle cy = "12" r = "2.5" cx = "4.5" ></ circle > < path d = "M7 12h10" ></ path > < circle r = "2.5" cx = "19.5" cy = "12" ></ circle > < path d = "m13.8 17.7 3.9-3.9" ></ path > < circle r = "2.5" cy = "19.5" cx = "12" ></ circle > < / > } } pub const LucideWaypoints : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;