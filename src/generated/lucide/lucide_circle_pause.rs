use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < line y2 = "9" x1 = "10" x2 = "10" y1 = "15" ></ line > < line y1 = "15" x1 = "14" y2 = "9" x2 = "14" ></ line > < / > } } pub const LUCIDE_CIRCLE_PAUSE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;