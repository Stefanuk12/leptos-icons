use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "10" x2 = "14" y2 = "2" y1 = "2" ></ line > < line x1 = "12" x2 = "15" y1 = "14" y2 = "11" ></ line > < circle cx = "12" cy = "14" r = "8" ></ circle > < / > } } pub const LUCIDE_TIMER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none")] } ;