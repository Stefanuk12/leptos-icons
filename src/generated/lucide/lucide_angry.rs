use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < path d = "M16 16s-1.5-2-4-2-4 2-4 2" ></ path > < path d = "M7.5 8 10 9" ></ path > < path d = "m14 9 2.5-1" ></ path > < path d = "M9 10h.01" ></ path > < path d = "M15 10h.01" ></ path > < / > } } pub const LUCIDE_ANGRY : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;