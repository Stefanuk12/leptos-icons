use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line x1 = "12" y1 = "8" y2 = "12" x2 = "12" ></ line > < line x1 = "12" y1 = "16" y2 = "16" x2 = "12.01" ></ line > < / > } } pub const LUCIDE_CIRCLE_ALERT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;