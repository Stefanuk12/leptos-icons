use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line y1 = "12" x2 = "18" x1 = "22" y2 = "12" ></ line > < line y1 = "12" y2 = "12" x1 = "6" x2 = "2" ></ line > < line x1 = "12" y2 = "2" x2 = "12" y1 = "6" ></ line > < line x1 = "12" y2 = "18" y1 = "22" x2 = "12" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;