use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < circle r = "4" cy = "12" cx = "12" ></ circle > < line y2 = "8" y1 = "8" x2 = "12" x1 = "21.17" ></ line > < line x2 = "8.54" y1 = "6.06" x1 = "3.95" y2 = "14" ></ line > < line x2 = "15.46" x1 = "10.88" y2 = "14" y1 = "21.94" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none")] } ;