use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line x2 = "18" x1 = "22" y2 = "12" y1 = "12" ></ line > < line y2 = "12" x2 = "2" x1 = "6" y1 = "12" ></ line > < line y1 = "6" x2 = "12" y2 = "2" x1 = "12" ></ line > < line y1 = "22" y2 = "18" x1 = "12" x2 = "12" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;