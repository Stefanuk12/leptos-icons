use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 16v2" ></ path > < path d = "M19 16v2" ></ path > < rect width = "20" x = "2" rx = "2" height = "8" y = "8" ></ rect > < path d = "M18 12h.01" ></ path > < / > } } pub const LUCIDE_RADIO_RECEIVER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;