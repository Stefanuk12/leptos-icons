use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 11.5V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1.4" ></ path > < path d = "M14 10V8a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v2" ></ path > < path d = "M10 9.9V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v5" ></ path > < path d = "M6 14v0a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0" ></ path > < path d = "M18 11v0a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-4a8 8 0 0 1-8-8 2 2 0 1 1 4 0" ></ path > < / > } } pub const LucideGrab : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;