use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 16v2" ></ path > < path d = "M19 16v2" ></ path > < rect height = "8" x = "2" rx = "2" width = "20" y = "8" ></ rect > < path d = "M18 12h0" ></ path > < / > } } pub const LUCIDE_RADIO_RECEIVER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;