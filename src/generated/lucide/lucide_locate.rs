use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "5" y1 = "12" y2 = "12" x1 = "2" ></ line > < line x1 = "19" y2 = "12" x2 = "22" y1 = "12" ></ line > < line x1 = "12" x2 = "12" y1 = "2" y2 = "5" ></ line > < line y2 = "22" x2 = "12" y1 = "19" x1 = "12" ></ line > < circle cx = "12" r = "7" cy = "12" ></ circle > < / > } } pub const LucideLocate : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;