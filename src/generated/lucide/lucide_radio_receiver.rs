use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 16v2" ></ path > < path d = "M19 16v2" ></ path > < rect height = "8" rx = "2" width = "20" y = "8" x = "2" ></ rect > < path d = "M18 12h0" ></ path > < / > } } pub const LUCIDE_RADIO_RECEIVER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;