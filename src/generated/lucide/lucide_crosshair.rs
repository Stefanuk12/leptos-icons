use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < line y2 = "12" y1 = "12" x1 = "22" x2 = "18" ></ line > < line x1 = "6" y2 = "12" x2 = "2" y1 = "12" ></ line > < line y1 = "6" y2 = "2" x1 = "12" x2 = "12" ></ line > < line y1 = "22" y2 = "18" x1 = "12" x2 = "12" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;