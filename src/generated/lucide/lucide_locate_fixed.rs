use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" y2 = "12" x2 = "5" x1 = "2" ></ line > < line y1 = "12" x2 = "22" x1 = "19" y2 = "12" ></ line > < line y1 = "2" x2 = "12" x1 = "12" y2 = "5" ></ line > < line x1 = "12" x2 = "12" y2 = "22" y1 = "19" ></ line > < circle cy = "12" cx = "12" r = "7" ></ circle > < circle cy = "12" cx = "12" r = "3" ></ circle > < / > } } pub const LucideLocateFixed : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;