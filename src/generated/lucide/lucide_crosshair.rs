use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line y2 = "12" y1 = "12" x2 = "18" x1 = "22" ></ line > < line y1 = "12" x2 = "2" y2 = "12" x1 = "6" ></ line > < line x1 = "12" y2 = "2" x2 = "12" y1 = "6" ></ line > < line x1 = "12" y2 = "18" x2 = "12" y1 = "22" ></ line > < / > } } pub const LucideCrosshair : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;