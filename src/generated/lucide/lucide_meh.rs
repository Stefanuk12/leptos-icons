use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < line x2 = "16" y1 = "15" y2 = "15" x1 = "8" ></ line > < line x1 = "9" y1 = "9" x2 = "9.01" y2 = "9" ></ line > < line y1 = "9" y2 = "9" x1 = "15" x2 = "15.01" ></ line > < / > } } pub const LUCIDE_MEH : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;