use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < circle cy = "12" r = "4" cx = "12" ></ circle > < line x1 = "21.17" y1 = "8" y2 = "8" x2 = "12" ></ line > < line y1 = "6.06" x1 = "3.95" y2 = "14" x2 = "8.54" ></ line > < line y2 = "14" x1 = "10.88" y1 = "21.94" x2 = "15.46" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;