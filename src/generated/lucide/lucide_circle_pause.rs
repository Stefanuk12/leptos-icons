use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < line x1 = "10" x2 = "10" y1 = "15" y2 = "9" ></ line > < line x1 = "14" x2 = "14" y2 = "9" y1 = "15" ></ line > < / > } } pub const LUCIDE_CIRCLE_PAUSE : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24")] } ;