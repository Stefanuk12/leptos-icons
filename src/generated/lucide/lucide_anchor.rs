use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 22V8" ></ path > < path d = "M5 12H2a10 10 0 0 0 20 0h-3" ></ path > < circle cx = "12" r = "3" cy = "5" ></ circle > < / > } } pub const LUCIDE_ANCHOR : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;