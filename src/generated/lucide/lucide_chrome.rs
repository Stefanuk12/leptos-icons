use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < circle cy = "12" cx = "12" r = "4" ></ circle > < line x1 = "21.17" x2 = "12" y2 = "8" y1 = "8" ></ line > < line y2 = "14" x1 = "3.95" y1 = "6.06" x2 = "8.54" ></ line > < line y2 = "14" x2 = "15.46" y1 = "21.94" x1 = "10.88" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;