use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x1 = "2" y2 = "12" x2 = "5" ></ line > < line y1 = "12" y2 = "12" x1 = "19" x2 = "22" ></ line > < line y2 = "5" x1 = "12" x2 = "12" y1 = "2" ></ line > < line x1 = "12" y1 = "19" y2 = "22" x2 = "12" ></ line > < circle r = "7" cy = "12" cx = "12" ></ circle > < circle cx = "12" cy = "12" r = "3" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24")] } ;