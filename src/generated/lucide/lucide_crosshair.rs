use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line x1 = "22" x2 = "18" y2 = "12" y1 = "12" ></ line > < line x1 = "6" y1 = "12" y2 = "12" x2 = "2" ></ line > < line x1 = "12" y2 = "2" y1 = "6" x2 = "12" ></ line > < line x2 = "12" x1 = "12" y2 = "18" y1 = "22" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;