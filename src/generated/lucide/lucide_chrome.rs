use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < circle cx = "12" cy = "12" r = "4" ></ circle > < line x1 = "21.17" y1 = "8" x2 = "12" y2 = "8" ></ line > < line x1 = "3.95" y2 = "14" x2 = "8.54" y1 = "6.06" ></ line > < line x1 = "10.88" y2 = "14" x2 = "15.46" y1 = "21.94" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;