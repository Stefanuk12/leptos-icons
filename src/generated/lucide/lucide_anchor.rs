use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 22V8" ></ path > < path d = "M5 12H2a10 10 0 0 0 20 0h-3" ></ path > < circle r = "3" cy = "5" cx = "12" ></ circle > < / > } } pub const LUCIDE_ANCHOR : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;