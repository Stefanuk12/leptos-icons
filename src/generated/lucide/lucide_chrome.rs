use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < circle r = "4" cy = "12" cx = "12" ></ circle > < line x1 = "21.17" x2 = "12" y1 = "8" y2 = "8" ></ line > < line y1 = "6.06" x1 = "3.95" x2 = "8.54" y2 = "14" ></ line > < line x2 = "15.46" y2 = "14" x1 = "10.88" y1 = "21.94" ></ line > < / > } } pub const LucideChrome : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;