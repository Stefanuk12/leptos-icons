use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" rx = "2" y = "4" width = "18" x = "3" ></ rect > < path d = "M16 2v4" ></ path > < path d = "M3 10h18" ></ path > < path d = "M8 2v4" ></ path > < path d = "M17 14h-6" ></ path > < path d = "M13 18H7" ></ path > < path d = "M7 14h.01" ></ path > < path d = "M17 18h.01" ></ path > < / > } } pub const LUCIDE_CALENDAR_RANGE : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;