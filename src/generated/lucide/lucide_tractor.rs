use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m10 11 11 .9a1 1 0 0 1 .8 1.1l-.665 4.158a1 1 0 0 1-.988.842H20" ></ path > < path d = "M16 18h-5" ></ path > < path d = "M18 5a1 1 0 0 0-1 1v5.573" ></ path > < path d = "M3 4h8.129a1 1 0 0 1 .99.863L13 11.246" ></ path > < path d = "M4 11V4" ></ path > < path d = "M7 15h.01" ></ path > < path d = "M8 10.1V4" ></ path > < circle cy = "18" r = "2" cx = "18" ></ circle > < circle cx = "7" cy = "15" r = "5" ></ circle > < / > } } pub const LUCIDE_TRACTOR : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none")] } ;