use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect height = "18" x = "3" width = "18" y = "4" rx = "2" ></ rect > < path d = "M3 10h18" ></ path > < path d = "M8 14h.01" ></ path > < path d = "M12 14h.01" ></ path > < path d = "M16 14h.01" ></ path > < path d = "M8 18h.01" ></ path > < path d = "M12 18h.01" ></ path > < path d = "M16 18h.01" ></ path > < / > } } pub const LUCIDE_CALENDAR_DAYS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;