use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < line y1 = "15" x2 = "10" x1 = "10" y2 = "9" ></ line > < line x1 = "14" y2 = "9" y1 = "15" x2 = "14" ></ line > < / > } } pub const LUCIDE_CIRCLE_PAUSE : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;