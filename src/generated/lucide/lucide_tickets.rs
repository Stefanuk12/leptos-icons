use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4.5 8 10.58-5.06a1 1 0 0 1 1.342.488L18.5 8" ></ path > < path d = "M6 10V8" ></ path > < path d = "M6 14v1" ></ path > < path d = "M6 19v2" ></ path > < rect rx = "2" y = "8" x = "2" height = "13" width = "20" ></ rect > < / > } } pub const LUCIDE_TICKETS : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;