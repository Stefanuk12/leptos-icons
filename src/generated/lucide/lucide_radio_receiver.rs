use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 16v2" ></ path > < path d = "M19 16v2" ></ path > < rect width = "20" x = "2" height = "8" rx = "2" y = "8" ></ rect > < path d = "M18 12h.01" ></ path > < / > } } pub const LUCIDE_RADIO_RECEIVER : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;