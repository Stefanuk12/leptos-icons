use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" height = "12" width = "12" rx = "2" y = "10" x = "2" ></ rect > < path d = "m17.92 14 3.5-3.5a2.24 2.24 0 0 0 0-3l-5-4.92a2.24 2.24 0 0 0-3 0L10 6" ></ path > < path d = "M6 18h.01" ></ path > < path d = "M10 14h.01" ></ path > < path d = "M15 6h.01" ></ path > < path d = "M18 9h.01" ></ path > < / > } } pub const LUCIDE_DICES : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;