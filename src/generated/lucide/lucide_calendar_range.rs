use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "18" height = "18" y = "4" x = "3" ></ rect > < path d = "M16 2v4" ></ path > < path d = "M3 10h18" ></ path > < path d = "M8 2v4" ></ path > < path d = "M17 14h-6" ></ path > < path d = "M13 18H7" ></ path > < path d = "M7 14h.01" ></ path > < path d = "M17 18h.01" ></ path > < / > } } pub const LUCIDE_CALENDAR_RANGE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;