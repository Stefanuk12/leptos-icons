use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x1 = "2" y2 = "12" x2 = "5" ></ line > < line x1 = "19" y1 = "12" y2 = "12" x2 = "22" ></ line > < line x2 = "12" x1 = "12" y1 = "2" y2 = "5" ></ line > < line y2 = "22" x2 = "12" y1 = "19" x1 = "12" ></ line > < circle cy = "12" r = "7" cx = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24")] } ;