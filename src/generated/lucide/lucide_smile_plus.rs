use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 11v1a10 10 0 1 1-9-10" ></ path > < path d = "M8 14s1.5 2 4 2 4-2 4-2" ></ path > < line y1 = "9" x2 = "9.01" y2 = "9" x1 = "9" ></ line > < line y2 = "9" x1 = "15" y1 = "9" x2 = "15.01" ></ line > < path d = "M16 5h6" ></ path > < path d = "M19 2v6" ></ path > < / > } } pub const LUCIDE_SMILE_PLUS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24")] } ;