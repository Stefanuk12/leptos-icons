use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "9" y2 = "20" x1 = "9" y1 = "4" ></ line > < path d = "M4 7c0-1.7 1.3-3 3-3h13" ></ path > < path d = "M18 20c-1.7 0-3-1.3-3-3V4" ></ path > < / > } } pub const LUCIDE_PI : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;