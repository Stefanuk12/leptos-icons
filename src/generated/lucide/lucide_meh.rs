use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line y1 = "15" x1 = "8" x2 = "16" y2 = "15" ></ line > < line x2 = "9.01" y1 = "9" x1 = "9" y2 = "9" ></ line > < line y1 = "9" x2 = "15.01" y2 = "9" x1 = "15" ></ line > < / > } } pub const LUCIDE_MEH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;