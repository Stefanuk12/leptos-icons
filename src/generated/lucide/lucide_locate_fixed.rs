use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" y1 = "12" x2 = "5" y2 = "12" ></ line > < line x2 = "22" x1 = "19" y1 = "12" y2 = "12" ></ line > < line y1 = "2" x1 = "12" y2 = "5" x2 = "12" ></ line > < line y1 = "19" x2 = "12" x1 = "12" y2 = "22" ></ line > < circle cx = "12" cy = "12" r = "7" ></ circle > < circle cx = "12" cy = "12" r = "3" ></ circle > < / > } } pub const LUCIDE_LOCATE_FIXED : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round")] } ;