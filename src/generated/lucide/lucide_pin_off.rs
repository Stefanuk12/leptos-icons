use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" x2 = "22" y1 = "2" y2 = "22" ></ line > < line y2 = "22" x1 = "12" x2 = "12" y1 = "17" ></ line > < path d = "M9 9v1.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V17h12" ></ path > < path d = "M15 9.34V6h1a2 2 0 0 0 0-4H7.89" ></ path > < / > } } pub const LUCIDE_PIN_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;