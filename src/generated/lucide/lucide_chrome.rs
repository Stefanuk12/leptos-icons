use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < circle cx = "12" cy = "12" r = "4" ></ circle > < line x2 = "12" y2 = "8" y1 = "8" x1 = "21.17" ></ line > < line x2 = "8.54" x1 = "3.95" y2 = "14" y1 = "6.06" ></ line > < line x1 = "10.88" y1 = "21.94" y2 = "14" x2 = "15.46" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;