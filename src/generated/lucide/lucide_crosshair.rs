use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < line y2 = "12" x1 = "22" y1 = "12" x2 = "18" ></ line > < line x1 = "6" x2 = "2" y1 = "12" y2 = "12" ></ line > < line x2 = "12" x1 = "12" y2 = "2" y1 = "6" ></ line > < line x1 = "12" y1 = "22" x2 = "12" y2 = "18" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;