use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" y2 = "12" x2 = "5" x1 = "2" ></ line > < line x1 = "19" x2 = "22" y1 = "12" y2 = "12" ></ line > < line x1 = "12" y1 = "2" x2 = "12" y2 = "5" ></ line > < line x2 = "12" y2 = "22" x1 = "12" y1 = "19" ></ line > < circle cy = "12" r = "7" cx = "12" ></ circle > < circle cx = "12" r = "3" cy = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;