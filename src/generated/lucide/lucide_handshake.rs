use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m11 17 2 2a1 1 0 1 0 3-3" ></ path > < path d = "m14 14 2.5 2.5a1 1 0 1 0 3-3l-3.88-3.88a3 3 0 0 0-4.24 0l-.88.88a1 1 0 1 1-3-3l2.81-2.81a5.79 5.79 0 0 1 7.06-.87l.47.28a2 2 0 0 0 1.42.25L21 4" ></ path > < path d = "m21 3 1 11h-2" ></ path > < path d = "M3 3 2 14l6.5 6.5a1 1 0 1 0 3-3" ></ path > < path d = "M3 4h8" ></ path > < / > } } pub const LUCIDE_HANDSHAKE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;