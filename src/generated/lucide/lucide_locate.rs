use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" x2 = "5" y2 = "12" y1 = "12" ></ line > < line y2 = "12" x1 = "19" y1 = "12" x2 = "22" ></ line > < line y2 = "5" x1 = "12" x2 = "12" y1 = "2" ></ line > < line x2 = "12" y2 = "22" x1 = "12" y1 = "19" ></ line > < circle cx = "12" cy = "12" r = "7" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24")] } ;