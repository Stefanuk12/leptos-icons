use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line y2 = "15" x1 = "8" y1 = "15" x2 = "16" ></ line > < line y1 = "9" x2 = "9.01" x1 = "9" y2 = "9" ></ line > < line y1 = "9" x2 = "15.01" y2 = "9" x1 = "15" ></ line > < / > } } pub const LUCIDE_MEH : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round")] } ;