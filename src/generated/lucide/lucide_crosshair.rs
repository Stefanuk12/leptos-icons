use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < line x2 = "18" y2 = "12" x1 = "22" y1 = "12" ></ line > < line x2 = "2" y1 = "12" y2 = "12" x1 = "6" ></ line > < line y1 = "6" y2 = "2" x1 = "12" x2 = "12" ></ line > < line x1 = "12" y2 = "18" x2 = "12" y1 = "22" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24")] } ;