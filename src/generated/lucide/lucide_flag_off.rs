use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2c3 0 5 2 8 2s4-1 4-1v11" ></ path > < path d = "M4 22V4" ></ path > < path d = "M4 15s1-1 4-1 5 2 8 2" ></ path > < line y2 = "22" y1 = "2" x1 = "2" x2 = "22" ></ line > < / > } } pub const LUCIDE_FLAG_OFF : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none")] } ;