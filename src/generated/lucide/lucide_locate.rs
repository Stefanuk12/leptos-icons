use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" y1 = "12" x1 = "2" x2 = "5" ></ line > < line y1 = "12" y2 = "12" x2 = "22" x1 = "19" ></ line > < line y1 = "2" y2 = "5" x1 = "12" x2 = "12" ></ line > < line y1 = "19" x1 = "12" x2 = "12" y2 = "22" ></ line > < circle r = "7" cy = "12" cx = "12" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24")] } ;