use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "18" rx = "2" y = "4" width = "18" ></ rect > < path d = "M16 2v4" ></ path > < path d = "M3 10h18" ></ path > < path d = "M8 2v4" ></ path > < path d = "M17 14h-6" ></ path > < path d = "M13 18H7" ></ path > < path d = "M7 14h.01" ></ path > < path d = "M17 18h.01" ></ path > < / > } } pub const LUCIDE_CALENDAR_RANGE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;