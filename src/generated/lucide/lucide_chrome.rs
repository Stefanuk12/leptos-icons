use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < circle r = "4" cx = "12" cy = "12" ></ circle > < line y1 = "8" x2 = "12" x1 = "21.17" y2 = "8" ></ line > < line y1 = "6.06" x2 = "8.54" x1 = "3.95" y2 = "14" ></ line > < line y1 = "21.94" y2 = "14" x1 = "10.88" x2 = "15.46" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;