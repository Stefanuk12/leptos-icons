use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 4c0-1.1.9-2 2-2" ></ path > < path d = "M20 2c1.1 0 2 .9 2 2" ></ path > < path d = "M22 8c0 1.1-.9 2-2 2" ></ path > < path d = "M16 10c-1.1 0-2-.9-2-2" ></ path > < path d = "m3 7 3 3 3-3" ></ path > < path d = "M6 10V5c0-1.7 1.3-3 3-3h1" ></ path > < rect x = "2" y = "14" height = "8" rx = "2" width = "8" ></ rect > < / > } } pub const LUCIDE_REPLACE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;