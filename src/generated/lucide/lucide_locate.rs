use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" x2 = "5" x1 = "2" y1 = "12" ></ line > < line x2 = "22" y1 = "12" x1 = "19" y2 = "12" ></ line > < line y2 = "5" y1 = "2" x1 = "12" x2 = "12" ></ line > < line x1 = "12" y1 = "19" y2 = "22" x2 = "12" ></ line > < circle cy = "12" cx = "12" r = "7" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;