use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" y2 = "22" x2 = "22" y1 = "2" ></ line > < line x1 = "12" y2 = "22" x2 = "12" y1 = "17" ></ line > < path d = "M9 9v1.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V17h12" ></ path > < path d = "M15 9.34V6h1a2 2 0 0 0 0-4H7.89" ></ path > < / > } } pub const LUCIDE_PIN_OFF : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;