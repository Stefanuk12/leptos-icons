use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" x = "4" y = "2" width = "16" rx = "2" ></ rect > < line y2 = "6" y1 = "6" x1 = "8" x2 = "16" ></ line > < line y2 = "18" x1 = "16" x2 = "16" y1 = "14" ></ line > < path d = "M16 10h.01" ></ path > < path d = "M12 10h.01" ></ path > < path d = "M8 10h.01" ></ path > < path d = "M12 14h.01" ></ path > < path d = "M8 14h.01" ></ path > < path d = "M12 18h.01" ></ path > < path d = "M8 18h.01" ></ path > < / > } } pub const LUCIDE_CALCULATOR : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24")] } ;