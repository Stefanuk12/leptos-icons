use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" y = "2" x = "4" rx = "2" width = "16" ></ rect > < line x1 = "8" y2 = "6" x2 = "16" y1 = "6" ></ line > < line x1 = "16" x2 = "16" y2 = "18" y1 = "14" ></ line > < path d = "M16 10h.01" ></ path > < path d = "M12 10h.01" ></ path > < path d = "M8 10h.01" ></ path > < path d = "M12 14h.01" ></ path > < path d = "M8 14h.01" ></ path > < path d = "M12 18h.01" ></ path > < path d = "M8 18h.01" ></ path > < / > } } pub const LUCIDE_CALCULATOR : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;