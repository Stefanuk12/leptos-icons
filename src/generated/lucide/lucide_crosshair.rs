use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line x2 = "18" y1 = "12" y2 = "12" x1 = "22" ></ line > < line y1 = "12" x2 = "2" y2 = "12" x1 = "6" ></ line > < line x2 = "12" y2 = "2" x1 = "12" y1 = "6" ></ line > < line x2 = "12" y2 = "18" y1 = "22" x1 = "12" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24")] } ;