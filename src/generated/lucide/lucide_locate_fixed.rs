use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" y1 = "12" x1 = "2" x2 = "5" ></ line > < line x2 = "22" x1 = "19" y2 = "12" y1 = "12" ></ line > < line y1 = "2" x1 = "12" y2 = "5" x2 = "12" ></ line > < line y1 = "19" x1 = "12" y2 = "22" x2 = "12" ></ line > < circle cx = "12" cy = "12" r = "7" ></ circle > < circle r = "3" cx = "12" cy = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;