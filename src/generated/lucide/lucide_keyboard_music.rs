use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" x = "2" height = "16" width = "20" rx = "2" ></ rect > < path d = "M6 8h4" ></ path > < path d = "M14 8h.01" ></ path > < path d = "M18 8h.01" ></ path > < path d = "M2 12h20" ></ path > < path d = "M6 12v4" ></ path > < path d = "M10 12v4" ></ path > < path d = "M14 12v4" ></ path > < path d = "M18 12v4" ></ path > < / > } } pub const LUCIDE_KEYBOARD_MUSIC : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;