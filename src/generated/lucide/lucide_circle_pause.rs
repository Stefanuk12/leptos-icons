use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line x1 = "10" y1 = "15" x2 = "10" y2 = "9" ></ line > < line x2 = "14" x1 = "14" y2 = "9" y1 = "15" ></ line > < / > } } pub const LUCIDE_CIRCLE_PAUSE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;