use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < line y1 = "8" x2 = "12" x1 = "12" y2 = "12" ></ line > < line x1 = "12" y2 = "16" x2 = "12.01" y1 = "16" ></ line > < / > } } pub const LUCIDE_CIRCLE_ALERT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;