use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < circle cy = "12" r = "4" cx = "12" ></ circle > < line y1 = "8" x1 = "21.17" x2 = "12" y2 = "8" ></ line > < line y1 = "6.06" x2 = "8.54" y2 = "14" x1 = "3.95" ></ line > < line y2 = "14" x1 = "10.88" x2 = "15.46" y1 = "21.94" ></ line > < / > } } pub const LucideChrome : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;