use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "5" y1 = "12" y2 = "12" x1 = "2" ></ line > < line x1 = "19" x2 = "22" y1 = "12" y2 = "12" ></ line > < line x1 = "12" x2 = "12" y1 = "2" y2 = "5" ></ line > < line y1 = "19" x1 = "12" y2 = "22" x2 = "12" ></ line > < circle cy = "12" cx = "12" r = "7" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;