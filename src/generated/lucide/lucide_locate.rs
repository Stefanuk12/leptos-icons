use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" y2 = "12" x2 = "5" y1 = "12" ></ line > < line x2 = "22" y1 = "12" y2 = "12" x1 = "19" ></ line > < line x2 = "12" y2 = "5" y1 = "2" x1 = "12" ></ line > < line y2 = "22" x2 = "12" x1 = "12" y1 = "19" ></ line > < circle cy = "12" r = "7" cx = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24")] } ;