use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < circle r = "4" cy = "12" cx = "12" ></ circle > < line y1 = "8" x1 = "21.17" y2 = "8" x2 = "12" ></ line > < line y2 = "14" x2 = "8.54" y1 = "6.06" x1 = "3.95" ></ line > < line y1 = "21.94" x2 = "15.46" y2 = "14" x1 = "10.88" ></ line > < / > } } pub const LucideChrome : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;