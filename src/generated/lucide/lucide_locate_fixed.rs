use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "5" x1 = "2" y1 = "12" y2 = "12" ></ line > < line y2 = "12" x1 = "19" y1 = "12" x2 = "22" ></ line > < line y2 = "5" x2 = "12" y1 = "2" x1 = "12" ></ line > < line y2 = "22" x1 = "12" x2 = "12" y1 = "19" ></ line > < circle cx = "12" cy = "12" r = "7" ></ circle > < circle r = "3" cy = "12" cx = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;