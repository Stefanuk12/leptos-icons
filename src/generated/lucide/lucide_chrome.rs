use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < circle r = "4" cy = "12" cx = "12" ></ circle > < line x2 = "12" x1 = "21.17" y2 = "8" y1 = "8" ></ line > < line y2 = "14" x1 = "3.95" y1 = "6.06" x2 = "8.54" ></ line > < line x1 = "10.88" y2 = "14" x2 = "15.46" y1 = "21.94" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;