use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < path d = "M12 2a7 7 0 1 0 10 10" ></ path > < / > } } pub const LUCIDE_ECLIPSE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;