use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 8h.01" ></ path > < path d = "M12 12h.01" ></ path > < path d = "M14 8h.01" ></ path > < path d = "M16 12h.01" ></ path > < path d = "M18 8h.01" ></ path > < path d = "M6 8h.01" ></ path > < path d = "M7 16h10" ></ path > < path d = "M8 12h.01" ></ path > < rect x = "2" height = "16" rx = "2" width = "20" y = "4" ></ rect > < / > } } pub const LUCIDE_KEYBOARD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24")] } ;