use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < line y1 = "12" x2 = "18" y2 = "12" x1 = "22" ></ line > < line x2 = "2" y2 = "12" y1 = "12" x1 = "6" ></ line > < line y2 = "2" x2 = "12" x1 = "12" y1 = "6" ></ line > < line y2 = "18" x2 = "12" x1 = "12" y1 = "22" ></ line > < / > } } pub const LUCIDE_CROSSHAIR : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;