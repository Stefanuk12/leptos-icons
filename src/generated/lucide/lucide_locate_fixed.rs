use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x2 = "5" x1 = "2" y2 = "12" ></ line > < line y2 = "12" x2 = "22" x1 = "19" y1 = "12" ></ line > < line x1 = "12" x2 = "12" y2 = "5" y1 = "2" ></ line > < line x2 = "12" y1 = "19" x1 = "12" y2 = "22" ></ line > < circle cy = "12" cx = "12" r = "7" ></ circle > < circle r = "3" cx = "12" cy = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;