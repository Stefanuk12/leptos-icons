use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19.3 14.8C20.1 16.4 21 17 21 17H3s3-2 3-9c0-3.3 2.7-6 6-6 1 0 1.9.2 2.8.7" ></ path > < path d = "M10.3 21a1.94 1.94 0 0 0 3.4 0" ></ path > < path d = "M15 8h6" ></ path > < path d = "M18 5v6" ></ path > < / > } } pub const LUCIDE_BELL_PLUS : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;