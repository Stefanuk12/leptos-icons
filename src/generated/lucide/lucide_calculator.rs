use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" rx = "2" x = "4" height = "20" y = "2" ></ rect > < line y1 = "6" y2 = "6" x1 = "8" x2 = "16" ></ line > < line x1 = "16" y1 = "14" x2 = "16" y2 = "18" ></ line > < path d = "M16 10h.01" ></ path > < path d = "M12 10h.01" ></ path > < path d = "M8 10h.01" ></ path > < path d = "M12 14h.01" ></ path > < path d = "M8 14h.01" ></ path > < path d = "M12 18h.01" ></ path > < path d = "M8 18h.01" ></ path > < / > } } pub const LUCIDE_CALCULATOR : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2")] } ;