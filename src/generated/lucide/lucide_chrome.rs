use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < circle cx = "12" cy = "12" r = "4" ></ circle > < line y1 = "8" x2 = "12" x1 = "21.17" y2 = "8" ></ line > < line x1 = "3.95" y1 = "6.06" x2 = "8.54" y2 = "14" ></ line > < line y2 = "14" y1 = "21.94" x1 = "10.88" x2 = "15.46" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2")] } ;