use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" y2 = "12" x2 = "5" x1 = "2" ></ line > < line x1 = "19" y2 = "12" y1 = "12" x2 = "22" ></ line > < line x1 = "12" x2 = "12" y2 = "5" y1 = "2" ></ line > < line y2 = "22" x2 = "12" x1 = "12" y1 = "19" ></ line > < circle cx = "12" cy = "12" r = "7" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none")] } ;