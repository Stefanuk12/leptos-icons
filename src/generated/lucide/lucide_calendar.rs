use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect x = "3" height = "18" rx = "2" width = "18" y = "4" ></ rect > < path d = "M3 10h18" ></ path > < / > } } pub const LUCIDE_CALENDAR : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;