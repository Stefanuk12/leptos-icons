use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < line y1 = "15" x2 = "16" x1 = "8" y2 = "15" ></ line > < line x2 = "9.01" y1 = "9" x1 = "9" y2 = "9" ></ line > < line x2 = "15.01" y2 = "9" y1 = "9" x1 = "15" ></ line > < / > } } pub const LUCIDE_MEH : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;