use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 4c0-1.1.9-2 2-2" ></ path > < path d = "M20 2c1.1 0 2 .9 2 2" ></ path > < path d = "M22 8c0 1.1-.9 2-2 2" ></ path > < path d = "M16 10c-1.1 0-2-.9-2-2" ></ path > < path d = "m3 7 3 3 3-3" ></ path > < path d = "M6 10V5c0-1.7 1.3-3 3-3h1" ></ path > < rect y = "14" x = "2" width = "8" height = "8" rx = "2" ></ rect > < / > } } pub const LUCIDE_REPLACE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor")] } ;