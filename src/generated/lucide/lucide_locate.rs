use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" x2 = "5" x1 = "2" y1 = "12" ></ line > < line x1 = "19" y2 = "12" y1 = "12" x2 = "22" ></ line > < line x1 = "12" y2 = "5" y1 = "2" x2 = "12" ></ line > < line y1 = "19" y2 = "22" x1 = "12" x2 = "12" ></ line > < circle cx = "12" r = "7" cy = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none")] } ;