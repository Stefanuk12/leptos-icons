use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x2 = "5" y2 = "12" x1 = "2" ></ line > < line x1 = "19" x2 = "22" y2 = "12" y1 = "12" ></ line > < line y1 = "2" x2 = "12" x1 = "12" y2 = "5" ></ line > < line x1 = "12" x2 = "12" y1 = "19" y2 = "22" ></ line > < circle r = "7" cx = "12" cy = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;