use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2c3 0 5 2 8 2s4-1 4-1v11" ></ path > < path d = "M4 22V4" ></ path > < path d = "M4 15s1-1 4-1 5 2 8 2" ></ path > < line x1 = "2" y2 = "22" x2 = "22" y1 = "2" ></ line > < / > } } pub const LUCIDE_FLAG_OFF : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;