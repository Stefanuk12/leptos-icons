use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < line x2 = "18" y1 = "12" y2 = "12" x1 = "22" ></ line > < line y2 = "12" x1 = "6" y1 = "12" x2 = "2" ></ line > < line x2 = "12" y1 = "6" y2 = "2" x1 = "12" ></ line > < line y2 = "18" x1 = "12" x2 = "12" y1 = "22" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;