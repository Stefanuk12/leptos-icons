use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < line y2 = "12" x2 = "18" x1 = "22" y1 = "12" ></ line > < line x2 = "2" y2 = "12" y1 = "12" x1 = "6" ></ line > < line y1 = "6" x1 = "12" y2 = "2" x2 = "12" ></ line > < line x1 = "12" x2 = "12" y2 = "18" y1 = "22" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round")] } ;