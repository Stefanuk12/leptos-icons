use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "5" y1 = "12" y2 = "12" x1 = "2" ></ line > < line y2 = "12" y1 = "12" x1 = "19" x2 = "22" ></ line > < line y1 = "2" x2 = "12" y2 = "5" x1 = "12" ></ line > < line x2 = "12" y2 = "22" x1 = "12" y1 = "19" ></ line > < circle cy = "12" cx = "12" r = "7" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24")] } ;