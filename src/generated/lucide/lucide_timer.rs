use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "14" x1 = "10" y2 = "2" y1 = "2" ></ line > < line x1 = "12" y1 = "14" x2 = "15" y2 = "11" ></ line > < circle cy = "14" r = "8" cx = "12" ></ circle > < / > } } pub const LUCIDE_TIMER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24")] } ;