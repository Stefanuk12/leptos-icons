use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 8h.01" ></ path > < path d = "M12 12h.01" ></ path > < path d = "M14 8h.01" ></ path > < path d = "M16 12h.01" ></ path > < path d = "M18 8h.01" ></ path > < path d = "M6 8h.01" ></ path > < path d = "M7 16h10" ></ path > < path d = "M8 12h.01" ></ path > < rect x = "2" width = "20" rx = "2" height = "16" y = "4" ></ rect > < / > } } pub const LUCIDE_KEYBOARD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;