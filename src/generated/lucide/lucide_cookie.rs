use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2a10 10 0 1 0 10 10 4 4 0 0 1-5-5 4 4 0 0 1-5-5" ></ path > < path d = "M8.5 8.5v.01" ></ path > < path d = "M16 15.5v.01" ></ path > < path d = "M12 12v.01" ></ path > < path d = "M11 17v.01" ></ path > < path d = "M7 14v.01" ></ path > < / > } } pub const LUCIDE_COOKIE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;