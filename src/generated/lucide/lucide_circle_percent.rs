use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < path d = "m15 9-6 6" ></ path > < path d = "M9 9h.01" ></ path > < path d = "M15 15h.01" ></ path > < / > } } pub const LUCIDE_CIRCLE_PERCENT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none")] } ;