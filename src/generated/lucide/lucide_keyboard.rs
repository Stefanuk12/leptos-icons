use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 8h.01" ></ path > < path d = "M12 12h.01" ></ path > < path d = "M14 8h.01" ></ path > < path d = "M16 12h.01" ></ path > < path d = "M18 8h.01" ></ path > < path d = "M6 8h.01" ></ path > < path d = "M7 16h10" ></ path > < path d = "M8 12h.01" ></ path > < rect height = "16" rx = "2" width = "20" y = "4" x = "2" ></ rect > < / > } } pub const LUCIDE_KEYBOARD : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;