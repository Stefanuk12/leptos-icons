use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < line y2 = "12" y1 = "12" x1 = "22" x2 = "18" ></ line > < line y2 = "12" y1 = "12" x2 = "2" x1 = "6" ></ line > < line y1 = "6" x1 = "12" x2 = "12" y2 = "2" ></ line > < line x1 = "12" x2 = "12" y2 = "18" y1 = "22" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;