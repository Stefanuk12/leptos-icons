use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < circle cy = "12" r = "4" cx = "12" ></ circle > < line y2 = "8" x1 = "21.17" y1 = "8" x2 = "12" ></ line > < line x2 = "8.54" y1 = "6.06" y2 = "14" x1 = "3.95" ></ line > < line y1 = "21.94" y2 = "14" x1 = "10.88" x2 = "15.46" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;