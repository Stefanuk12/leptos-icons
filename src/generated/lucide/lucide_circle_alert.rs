use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < line y1 = "8" y2 = "12" x1 = "12" x2 = "12" ></ line > < line y1 = "16" x2 = "12.01" y2 = "16" x1 = "12" ></ line > < / > } } pub const LUCIDE_CIRCLE_ALERT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24")] } ;