use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < line y2 = "12" x2 = "18" x1 = "22" y1 = "12" ></ line > < line x2 = "2" x1 = "6" y1 = "12" y2 = "12" ></ line > < line y1 = "6" x2 = "12" y2 = "2" x1 = "12" ></ line > < line y2 = "18" x2 = "12" y1 = "22" x1 = "12" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;