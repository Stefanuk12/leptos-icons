use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect width = "18" x = "3" rx = "2" y = "4" height = "18" ></ rect > < path d = "M3 10h18" ></ path > < path d = "M10 16h4" ></ path > < path d = "M12 14v4" ></ path > < / > } } pub const LUCIDE_CALENDAR_PLUS_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;