use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < circle r = "4" cy = "12" cx = "12" ></ circle > < line y1 = "8" y2 = "8" x1 = "21.17" x2 = "12" ></ line > < line x1 = "3.95" x2 = "8.54" y1 = "6.06" y2 = "14" ></ line > < line y2 = "14" x2 = "15.46" y1 = "21.94" x1 = "10.88" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;