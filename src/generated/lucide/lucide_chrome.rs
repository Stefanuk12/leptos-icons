use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < circle cx = "12" cy = "12" r = "4" ></ circle > < line y2 = "8" y1 = "8" x2 = "12" x1 = "21.17" ></ line > < line x1 = "3.95" x2 = "8.54" y2 = "14" y1 = "6.06" ></ line > < line x1 = "10.88" x2 = "15.46" y1 = "21.94" y2 = "14" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;