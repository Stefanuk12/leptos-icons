use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < circle cy = "12" r = "4" cx = "12" ></ circle > < line y1 = "8" y2 = "8" x2 = "12" x1 = "21.17" ></ line > < line y2 = "14" x1 = "3.95" x2 = "8.54" y1 = "6.06" ></ line > < line y1 = "21.94" x1 = "10.88" x2 = "15.46" y2 = "14" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;