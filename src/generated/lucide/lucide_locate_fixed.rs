use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "5" y1 = "12" y2 = "12" x1 = "2" ></ line > < line x2 = "22" y1 = "12" x1 = "19" y2 = "12" ></ line > < line x2 = "12" x1 = "12" y1 = "2" y2 = "5" ></ line > < line y1 = "19" x2 = "12" x1 = "12" y2 = "22" ></ line > < circle cx = "12" r = "7" cy = "12" ></ circle > < circle r = "3" cy = "12" cx = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none")] } ;