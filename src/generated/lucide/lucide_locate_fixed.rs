use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x1 = "2" x2 = "5" y2 = "12" ></ line > < line y2 = "12" x1 = "19" x2 = "22" y1 = "12" ></ line > < line x1 = "12" y1 = "2" x2 = "12" y2 = "5" ></ line > < line y2 = "22" y1 = "19" x1 = "12" x2 = "12" ></ line > < circle r = "7" cx = "12" cy = "12" ></ circle > < circle cx = "12" cy = "12" r = "3" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none")] } ;