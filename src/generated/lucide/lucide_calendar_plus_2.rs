use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect height = "18" y = "4" rx = "2" x = "3" width = "18" ></ rect > < path d = "M3 10h18" ></ path > < path d = "M10 16h4" ></ path > < path d = "M12 14v4" ></ path > < / > } } pub const LUCIDE_CALENDAR_PLUS_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;