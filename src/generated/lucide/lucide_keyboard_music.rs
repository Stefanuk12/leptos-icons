use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "20" x = "2" y = "4" height = "16" ></ rect > < path d = "M6 8h4" ></ path > < path d = "M14 8h.01" ></ path > < path d = "M18 8h.01" ></ path > < path d = "M2 12h20" ></ path > < path d = "M6 12v4" ></ path > < path d = "M10 12v4" ></ path > < path d = "M14 12v4" ></ path > < path d = "M18 12v4" ></ path > < / > } } pub const LUCIDE_KEYBOARD_MUSIC : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;