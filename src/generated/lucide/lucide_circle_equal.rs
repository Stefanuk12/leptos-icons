use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 10h10" ></ path > < path d = "M7 14h10" ></ path > < circle r = "10" cx = "12" cy = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_EQUAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor")] } ;