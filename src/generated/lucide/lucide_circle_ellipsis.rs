use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < path d = "M17 12h.01" ></ path > < path d = "M12 12h.01" ></ path > < path d = "M7 12h.01" ></ path > < / > } } pub const LUCIDE_CIRCLE_ELLIPSIS : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;