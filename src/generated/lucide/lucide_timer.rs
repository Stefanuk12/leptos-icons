use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "14" x1 = "10" y1 = "2" y2 = "2" ></ line > < line x2 = "15" y2 = "11" x1 = "12" y1 = "14" ></ line > < circle r = "8" cx = "12" cy = "14" ></ circle > < / > } } pub const LUCIDE_TIMER : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;