use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 2h13a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-5" ></ path > < path d = "M10 10 2.5 2.5C2 2 2 2.5 2 5v3a2 2 0 0 0 2 2h6z" ></ path > < path d = "M22 17v-1a2 2 0 0 0-2-2h-1" ></ path > < path d = "M4 14a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16.5l1-.5.5.5-8-8H4z" ></ path > < path d = "M6 18h.01" ></ path > < path d = "m2 2 20 20" ></ path > < / > } } pub const LUCIDE_SERVER_OFF : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;