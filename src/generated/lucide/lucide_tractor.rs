use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m10 11 11 .9a1 1 0 0 1 .8 1.1l-.665 4.158a1 1 0 0 1-.988.842H20" ></ path > < path d = "M16 18h-5" ></ path > < path d = "M18 5a1 1 0 0 0-1 1v5.573" ></ path > < path d = "M3 4h8.129a1 1 0 0 1 .99.863L13 11.246" ></ path > < path d = "M4 11V4" ></ path > < path d = "M7 15h.01" ></ path > < path d = "M8 10.1V4" ></ path > < circle cx = "18" r = "2" cy = "18" ></ circle > < circle cy = "15" cx = "7" r = "5" ></ circle > < / > } } pub const LUCIDE_TRACTOR : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;