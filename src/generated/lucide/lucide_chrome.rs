use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < circle cy = "12" r = "4" cx = "12" ></ circle > < line y2 = "8" y1 = "8" x1 = "21.17" x2 = "12" ></ line > < line y1 = "6.06" y2 = "14" x1 = "3.95" x2 = "8.54" ></ line > < line y1 = "21.94" x1 = "10.88" y2 = "14" x2 = "15.46" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;