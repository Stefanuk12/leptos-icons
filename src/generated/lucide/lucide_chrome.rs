use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < circle cy = "12" cx = "12" r = "4" ></ circle > < line x2 = "12" y2 = "8" y1 = "8" x1 = "21.17" ></ line > < line y1 = "6.06" x2 = "8.54" x1 = "3.95" y2 = "14" ></ line > < line x1 = "10.88" y2 = "14" x2 = "15.46" y1 = "21.94" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;