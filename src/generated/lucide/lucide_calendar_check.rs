use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect height = "18" rx = "2" width = "18" x = "3" y = "4" ></ rect > < path d = "M3 10h18" ></ path > < path d = "m9 16 2 2 4-4" ></ path > < / > } } pub const LucideCalendarCheck : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;