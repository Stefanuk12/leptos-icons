use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 7v4" ></ path > < path d = "M7.998 9.003a5 5 0 1 0 8-.005" ></ path > < circle cx = "12" cy = "12" r = "10" ></ circle > < / > } } pub const LUCIDE_CIRCLE_POWER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;