use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x2 = "5" x1 = "2" y2 = "12" ></ line > < line x2 = "22" y1 = "12" y2 = "12" x1 = "19" ></ line > < line x2 = "12" y1 = "2" y2 = "5" x1 = "12" ></ line > < line y1 = "19" y2 = "22" x2 = "12" x1 = "12" ></ line > < circle cx = "12" cy = "12" r = "7" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor")] } ;