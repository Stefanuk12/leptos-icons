use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "2" y1 = "2" x1 = "10" x2 = "14" ></ line > < line x1 = "12" y2 = "11" y1 = "14" x2 = "15" ></ line > < circle cx = "12" r = "8" cy = "14" ></ circle > < / > } } pub const LUCIDE_TIMER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;