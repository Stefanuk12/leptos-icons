use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 3a2 2 0 0 0-2 2" ></ path > < path d = "M19 3a2 2 0 0 1 2 2" ></ path > < path d = "m12 12 4 10 1.7-4.3L22 16Z" ></ path > < path d = "M5 21a2 2 0 0 1-2-2" ></ path > < path d = "M9 3h1" ></ path > < path d = "M9 21h2" ></ path > < path d = "M14 3h1" ></ path > < path d = "M3 9v1" ></ path > < path d = "M21 9v2" ></ path > < path d = "M3 14v1" ></ path > < / > } } pub const LUCIDE_SQUARE_DASHED_MOUSE_POINTER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;