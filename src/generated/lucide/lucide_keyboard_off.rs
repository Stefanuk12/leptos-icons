use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M 20 4 A2 2 0 0 1 22 6" ></ path > < path d = "M 22 6 L 22 16.41" ></ path > < path d = "M 7 16 L 16 16" ></ path > < path d = "M 9.69 4 L 20 4" ></ path > < path d = "M14 8h.01" ></ path > < path d = "M18 8h.01" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M20 20H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2" ></ path > < path d = "M6 8h.01" ></ path > < path d = "M8 12h.01" ></ path > < / > } } pub const LUCIDE_KEYBOARD_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;