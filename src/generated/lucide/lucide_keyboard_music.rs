use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" rx = "2" height = "16" x = "2" width = "20" ></ rect > < path d = "M6 8h4" ></ path > < path d = "M14 8h.01" ></ path > < path d = "M18 8h.01" ></ path > < path d = "M2 12h20" ></ path > < path d = "M6 12v4" ></ path > < path d = "M10 12v4" ></ path > < path d = "M14 12v4" ></ path > < path d = "M18 12v4" ></ path > < / > } } pub const LUCIDE_KEYBOARD_MUSIC : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;