use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < path d = "M17 12h.01" ></ path > < path d = "M12 12h.01" ></ path > < path d = "M7 12h.01" ></ path > < / > } } pub const LUCIDE_CIRCLE_ELLIPSIS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;