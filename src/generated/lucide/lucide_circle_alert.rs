use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < line y1 = "8" y2 = "12" x1 = "12" x2 = "12" ></ line > < line y2 = "16" x1 = "12" x2 = "12.01" y1 = "16" ></ line > < / > } } pub const LUCIDE_CIRCLE_ALERT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;