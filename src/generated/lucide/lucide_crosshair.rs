use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < line y1 = "12" x2 = "18" x1 = "22" y2 = "12" ></ line > < line y2 = "12" x1 = "6" y1 = "12" x2 = "2" ></ line > < line y2 = "2" x1 = "12" x2 = "12" y1 = "6" ></ line > < line x2 = "12" x1 = "12" y2 = "18" y1 = "22" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;