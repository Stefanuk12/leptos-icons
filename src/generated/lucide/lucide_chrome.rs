use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < circle cx = "12" r = "4" cy = "12" ></ circle > < line x1 = "21.17" y2 = "8" x2 = "12" y1 = "8" ></ line > < line x2 = "8.54" y1 = "6.06" y2 = "14" x1 = "3.95" ></ line > < line x1 = "10.88" x2 = "15.46" y2 = "14" y1 = "21.94" ></ line > < / > } } pub const LUCIDE_CHROME : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;