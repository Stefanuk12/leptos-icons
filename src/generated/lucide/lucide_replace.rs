use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 4a2 2 0 0 1 2-2" ></ path > < path d = "M16 10a2 2 0 0 1-2-2" ></ path > < path d = "M20 2a2 2 0 0 1 2 2" ></ path > < path d = "M22 8a2 2 0 0 1-2 2" ></ path > < path d = "m3 7 3 3 3-3" ></ path > < path d = "M6 10V5a3 3 0 0 1 3-3h1" ></ path > < rect x = "2" width = "8" height = "8" rx = "2" y = "14" ></ rect > < / > } } pub const LUCIDE_REPLACE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;