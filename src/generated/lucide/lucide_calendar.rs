use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect y = "4" width = "18" rx = "2" height = "18" x = "3" ></ rect > < path d = "M3 10h18" ></ path > < / > } } pub const LUCIDE_CALENDAR : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round")] } ;