use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" y2 = "12" y1 = "12" x2 = "5" ></ line > < line y2 = "12" x2 = "22" x1 = "19" y1 = "12" ></ line > < line y2 = "5" x2 = "12" y1 = "2" x1 = "12" ></ line > < line y1 = "19" y2 = "22" x2 = "12" x1 = "12" ></ line > < circle cx = "12" cy = "12" r = "7" ></ circle > < circle cy = "12" r = "3" cx = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;