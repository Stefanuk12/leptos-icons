use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line x2 = "18" x1 = "22" y1 = "12" y2 = "12" ></ line > < line y1 = "12" x1 = "6" x2 = "2" y2 = "12" ></ line > < line y2 = "2" y1 = "6" x2 = "12" x1 = "12" ></ line > < line x1 = "12" x2 = "12" y1 = "22" y2 = "18" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2")] } ;