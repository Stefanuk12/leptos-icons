use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 3a2 2 0 0 0-2 2" ></ path > < path d = "M19 3a2 2 0 0 1 2 2" ></ path > < path d = "M21 19a2 2 0 0 1-2 2" ></ path > < path d = "M5 21a2 2 0 0 1-2-2" ></ path > < path d = "M9 3h1" ></ path > < path d = "M9 21h1" ></ path > < path d = "M14 3h1" ></ path > < path d = "M14 21h1" ></ path > < path d = "M3 9v1" ></ path > < path d = "M21 9v1" ></ path > < path d = "M3 14v1" ></ path > < path d = "M21 14v1" ></ path > < / > } } pub const LUCIDE_BOX_SELECT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2")] } ;