use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" y2 = "12" x1 = "2" x2 = "5" ></ line > < line x2 = "22" y1 = "12" x1 = "19" y2 = "12" ></ line > < line x1 = "12" y1 = "2" x2 = "12" y2 = "5" ></ line > < line x1 = "12" x2 = "12" y1 = "19" y2 = "22" ></ line > < circle r = "7" cx = "12" cy = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;