use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 22V8" ></ path > < path d = "M5 12H2a10 10 0 0 0 20 0h-3" ></ path > < circle cx = "12" r = "3" cy = "5" ></ circle > < / > } } pub const LUCIDE_ANCHOR : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;