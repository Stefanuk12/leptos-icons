use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect x = "3" rx = "2" height = "18" y = "4" width = "18" ></ rect > < path d = "M3 10h18" ></ path > < path d = "M8 14h.01" ></ path > < path d = "M12 14h.01" ></ path > < path d = "M16 14h.01" ></ path > < path d = "M8 18h.01" ></ path > < path d = "M12 18h.01" ></ path > < path d = "M16 18h.01" ></ path > < / > } } pub const LucideCalendarDays : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;