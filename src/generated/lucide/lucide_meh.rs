use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < line y2 = "15" x2 = "16" x1 = "8" y1 = "15" ></ line > < line y1 = "9" x2 = "9.01" y2 = "9" x1 = "9" ></ line > < line y1 = "9" x2 = "15.01" y2 = "9" x1 = "15" ></ line > < / > } } pub const LUCIDE_MEH : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;