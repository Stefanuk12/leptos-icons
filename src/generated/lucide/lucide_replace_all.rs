use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 14a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" ></ path > < path d = "M14 4a2 2 0 0 1 2-2" ></ path > < path d = "M16 10a2 2 0 0 1-2-2" ></ path > < path d = "M20 14a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" ></ path > < path d = "M20 2a2 2 0 0 1 2 2" ></ path > < path d = "M22 8a2 2 0 0 1-2 2" ></ path > < path d = "m3 7 3 3 3-3" ></ path > < path d = "M6 10V5a 3 3 0 0 1 3-3h1" ></ path > < rect rx = "2" height = "8" x = "2" width = "8" y = "14" ></ rect > < / > } } pub const LUCIDE_REPLACE_ALL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;