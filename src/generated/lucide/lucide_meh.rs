use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < line x1 = "8" y2 = "15" x2 = "16" y1 = "15" ></ line > < line y2 = "9" x2 = "9.01" y1 = "9" x1 = "9" ></ line > < line x1 = "15" y1 = "9" y2 = "9" x2 = "15.01" ></ line > < / > } } pub const LUCIDE_MEH : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;