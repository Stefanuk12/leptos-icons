use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "5" y2 = "12" x1 = "2" y1 = "12" ></ line > < line x2 = "22" y2 = "12" y1 = "12" x1 = "19" ></ line > < line x2 = "12" y1 = "2" x1 = "12" y2 = "5" ></ line > < line x1 = "12" y2 = "22" y1 = "19" x2 = "12" ></ line > < circle cx = "12" r = "7" cy = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2")] } ;