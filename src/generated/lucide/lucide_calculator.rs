use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" rx = "2" width = "16" x = "4" y = "2" ></ rect > < line x1 = "8" x2 = "16" y1 = "6" y2 = "6" ></ line > < line x2 = "16" y1 = "14" x1 = "16" y2 = "18" ></ line > < path d = "M16 10h.01" ></ path > < path d = "M12 10h.01" ></ path > < path d = "M8 10h.01" ></ path > < path d = "M12 14h.01" ></ path > < path d = "M8 14h.01" ></ path > < path d = "M12 18h.01" ></ path > < path d = "M8 18h.01" ></ path > < / > } } pub const LUCIDE_CALCULATOR : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24")] } ;