use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < line x2 = "18" y2 = "12" y1 = "12" x1 = "22" ></ line > < line y1 = "12" y2 = "12" x2 = "2" x1 = "6" ></ line > < line x1 = "12" y1 = "6" x2 = "12" y2 = "2" ></ line > < line y1 = "22" x1 = "12" y2 = "18" x2 = "12" ></ line > < / > } } pub const LucideCrosshair : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;