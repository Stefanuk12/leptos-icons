use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 9c0 .6-.4 1-1 1H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9c.6 0 1 .4 1 1Z" ></ path > < path d = "M18 6h4" ></ path > < path d = "M14 4h3a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-3" ></ path > < path d = "m5 10-2 8" ></ path > < path d = "M12 10v3c0 .6-.4 1-1 1H8" ></ path > < path d = "m7 18 2-8" ></ path > < path d = "M5 22c-1.7 0-3-1.3-3-3 0-.6.4-1 1-1h7c.6 0 1 .4 1 1v2c0 .6-.4 1-1 1Z" ></ path > < / > } } pub const LUCIDE_DRILL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24")] } ;