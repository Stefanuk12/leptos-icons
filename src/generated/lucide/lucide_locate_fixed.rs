use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" x2 = "5" y1 = "12" y2 = "12" ></ line > < line y2 = "12" x1 = "19" y1 = "12" x2 = "22" ></ line > < line x2 = "12" y2 = "5" x1 = "12" y1 = "2" ></ line > < line y1 = "19" x1 = "12" y2 = "22" x2 = "12" ></ line > < circle cy = "12" r = "7" cx = "12" ></ circle > < circle cx = "12" cy = "12" r = "3" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;