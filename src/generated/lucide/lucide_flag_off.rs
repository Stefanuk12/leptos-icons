use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2c3 0 5 2 8 2s4-1 4-1v11" ></ path > < path d = "M4 22V4" ></ path > < path d = "M4 15s1-1 4-1 5 2 8 2" ></ path > < line y2 = "22" x2 = "22" x1 = "2" y1 = "2" ></ line > < / > } } pub const LUCIDE_FLAG_OFF : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;