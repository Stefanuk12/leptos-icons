use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4.5 8 10.58-5.06a1 1 0 0 1 1.342.488L18.5 8" ></ path > < path d = "M6 10V8" ></ path > < path d = "M6 14v1" ></ path > < path d = "M6 19v2" ></ path > < rect height = "13" width = "20" y = "8" rx = "2" x = "2" ></ rect > < / > } } pub const LUCIDE_TICKETS : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;