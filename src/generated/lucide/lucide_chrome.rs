use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < circle cy = "12" cx = "12" r = "4" ></ circle > < line x1 = "21.17" y1 = "8" x2 = "12" y2 = "8" ></ line > < line y2 = "14" y1 = "6.06" x1 = "3.95" x2 = "8.54" ></ line > < line y1 = "21.94" x2 = "15.46" x1 = "10.88" y2 = "14" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;