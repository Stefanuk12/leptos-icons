use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line y2 = "12" x2 = "18" x1 = "22" y1 = "12" ></ line > < line x2 = "2" y1 = "12" y2 = "12" x1 = "6" ></ line > < line x2 = "12" y1 = "6" x1 = "12" y2 = "2" ></ line > < line y2 = "18" y1 = "22" x1 = "12" x2 = "12" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24")] } ;