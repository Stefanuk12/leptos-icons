use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 2v4" ></ path > < path d = "M3 10h18" ></ path > < path d = "M8 2v4" ></ path > < path d = "M17 14h-6" ></ path > < path d = "M13 18H7" ></ path > < path d = "M7 14h.01" ></ path > < path d = "M17 18h.01" ></ path > < / > } } pub const LucideCalendarRange : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;