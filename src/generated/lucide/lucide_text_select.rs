use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 3a2 2 0 0 0-2 2" ></ path > < path d = "M19 3a2 2 0 0 1 2 2" ></ path > < path d = "M21 19a2 2 0 0 1-2 2" ></ path > < path d = "M5 21a2 2 0 0 1-2-2" ></ path > < path d = "M9 3h1" ></ path > < path d = "M9 21h1" ></ path > < path d = "M14 3h1" ></ path > < path d = "M14 21h1" ></ path > < path d = "M3 9v1" ></ path > < path d = "M21 9v1" ></ path > < path d = "M3 14v1" ></ path > < path d = "M21 14v1" ></ path > < line y2 = "8" x1 = "7" x2 = "15" y1 = "8" ></ line > < line x2 = "17" x1 = "7" y1 = "12" y2 = "12" ></ line > < line y2 = "16" x2 = "13" x1 = "7" y1 = "16" ></ line > < / > } } pub const LUCIDE_TEXT_SELECT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;