use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" y2 = "12" y1 = "12" x2 = "5" ></ line > < line x2 = "22" y1 = "12" x1 = "19" y2 = "12" ></ line > < line x2 = "12" y1 = "2" y2 = "5" x1 = "12" ></ line > < line y1 = "19" y2 = "22" x1 = "12" x2 = "12" ></ line > < circle cy = "12" r = "7" cx = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2")] } ;