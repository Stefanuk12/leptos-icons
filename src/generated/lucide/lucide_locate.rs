use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" y1 = "12" y2 = "12" x2 = "5" ></ line > < line x1 = "19" y2 = "12" x2 = "22" y1 = "12" ></ line > < line y1 = "2" x1 = "12" y2 = "5" x2 = "12" ></ line > < line y2 = "22" y1 = "19" x2 = "12" x1 = "12" ></ line > < circle cy = "12" cx = "12" r = "7" ></ circle > < / > } } pub const LUCIDE_LOCATE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;