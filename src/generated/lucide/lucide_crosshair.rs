use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < line x2 = "18" y2 = "12" y1 = "12" x1 = "22" ></ line > < line y2 = "12" x1 = "6" y1 = "12" x2 = "2" ></ line > < line x2 = "12" x1 = "12" y1 = "6" y2 = "2" ></ line > < line y2 = "18" y1 = "22" x2 = "12" x1 = "12" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;