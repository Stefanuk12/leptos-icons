use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "5" x1 = "2" y1 = "12" y2 = "12" ></ line > < line x1 = "19" y1 = "12" y2 = "12" x2 = "22" ></ line > < line x2 = "12" x1 = "12" y2 = "5" y1 = "2" ></ line > < line x2 = "12" x1 = "12" y2 = "22" y1 = "19" ></ line > < circle r = "7" cy = "12" cx = "12" ></ circle > < circle cx = "12" cy = "12" r = "3" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none")] } ;