use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect height = "18" width = "18" x = "3" y = "4" rx = "2" ></ rect > < path d = "M3 10h18" ></ path > < path d = "m14 14-4 4" ></ path > < path d = "m10 14 4 4" ></ path > < / > } } pub const LucideCalendarX : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;