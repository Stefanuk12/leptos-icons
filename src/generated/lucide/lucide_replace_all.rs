use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 4c0-1.1.9-2 2-2" ></ path > < path d = "M20 2c1.1 0 2 .9 2 2" ></ path > < path d = "M22 8c0 1.1-.9 2-2 2" ></ path > < path d = "M16 10c-1.1 0-2-.9-2-2" ></ path > < path d = "m3 7 3 3 3-3" ></ path > < path d = "M6 10V5c0-1.7 1.3-3 3-3h1" ></ path > < rect width = "8" x = "2" rx = "2" height = "8" y = "14" ></ rect > < path d = "M14 14c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" ></ path > < path d = "M20 14c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" ></ path > < / > } } pub const LUCIDE_REPLACE_ALL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;