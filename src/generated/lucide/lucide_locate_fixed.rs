use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x1 = "2" x2 = "5" y2 = "12" ></ line > < line x1 = "19" y1 = "12" y2 = "12" x2 = "22" ></ line > < line y2 = "5" x2 = "12" y1 = "2" x1 = "12" ></ line > < line x1 = "12" x2 = "12" y1 = "19" y2 = "22" ></ line > < circle r = "7" cy = "12" cx = "12" ></ circle > < circle r = "3" cy = "12" cx = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24")] } ;