use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < path d = "M16 16s-1.5-2-4-2-4 2-4 2" ></ path > < path d = "M7.5 8 10 9" ></ path > < path d = "m14 9 2.5-1" ></ path > < path d = "M9 10h.01" ></ path > < path d = "M15 10h.01" ></ path > < / > } } pub const LUCIDE_ANGRY : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;