use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < line y1 = "12" y2 = "12" x1 = "22" x2 = "18" ></ line > < line x2 = "2" y1 = "12" x1 = "6" y2 = "12" ></ line > < line y1 = "6" y2 = "2" x2 = "12" x1 = "12" ></ line > < line x1 = "12" y2 = "18" y1 = "22" x2 = "12" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;