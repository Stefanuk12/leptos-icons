use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m10 11 11 .9c.6 0 .9.5.8 1.1l-.8 5h-1" ></ path > < path d = "M16 18h-5" ></ path > < path d = "M18 5a1 1 0 0 0-1 1v5.573" ></ path > < path d = "M3 4h9l1 7.246" ></ path > < path d = "M4 11V4" ></ path > < path d = "M7 15h.01" ></ path > < path d = "M8 10.1V4" ></ path > < circle cy = "18" cx = "18" r = "2" ></ circle > < circle cx = "7" cy = "15" r = "5" ></ circle > < / > } } pub const LUCIDE_TRACTOR : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none")] } ;