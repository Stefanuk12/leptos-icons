use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line y1 = "8" y2 = "12" x1 = "12" x2 = "12" ></ line > < line y1 = "16" x1 = "12" x2 = "12.01" y2 = "16" ></ line > < / > } } pub const LUCIDE_CIRCLE_ALERT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;