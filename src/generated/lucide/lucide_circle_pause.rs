use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line y1 = "15" x2 = "10" x1 = "10" y2 = "9" ></ line > < line y2 = "9" x1 = "14" x2 = "14" y1 = "15" ></ line > < / > } } pub const LUCIDE_CIRCLE_PAUSE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;