use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 12.5V10a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1.4" ></ path > < path d = "M14 11V9a2 2 0 1 0-4 0v2" ></ path > < path d = "M10 10.5V5a2 2 0 1 0-4 0v9" ></ path > < path d = "m7 15-1.76-1.76a2 2 0 0 0-2.83 2.82l3.6 3.6C7.5 21.14 9.2 22 12 22h2a8 8 0 0 0 8-8V7a2 2 0 1 0-4 0v5" ></ path > < / > } } pub const LUCIDE_HAND_METAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;