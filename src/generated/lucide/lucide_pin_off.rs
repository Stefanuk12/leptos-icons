use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" y2 = "22" y1 = "2" x2 = "22" ></ line > < line x1 = "12" y2 = "22" x2 = "12" y1 = "17" ></ line > < path d = "M9 9v1.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V17h12" ></ path > < path d = "M15 9.34V6h1a2 2 0 0 0 0-4H7.89" ></ path > < / > } } pub const LUCIDE_PIN_OFF : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor")] } ;