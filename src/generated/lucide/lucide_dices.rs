use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "12" y = "10" rx = "2" ry = "2" x = "2" width = "12" ></ rect > < path d = "m17.92 14 3.5-3.5a2.24 2.24 0 0 0 0-3l-5-4.92a2.24 2.24 0 0 0-3 0L10 6" ></ path > < path d = "M6 18h.01" ></ path > < path d = "M10 14h.01" ></ path > < path d = "M15 6h.01" ></ path > < path d = "M18 9h.01" ></ path > < / > } } pub const LUCIDE_DICES : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;