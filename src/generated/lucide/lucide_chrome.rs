use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < circle cx = "12" r = "4" cy = "12" ></ circle > < line x1 = "21.17" x2 = "12" y1 = "8" y2 = "8" ></ line > < line y1 = "6.06" x1 = "3.95" x2 = "8.54" y2 = "14" ></ line > < line x1 = "10.88" x2 = "15.46" y1 = "21.94" y2 = "14" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;