use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < line x1 = "8" y1 = "15" x2 = "16" y2 = "15" ></ line > < line x1 = "9" x2 = "9.01" y2 = "9" y1 = "9" ></ line > < line x1 = "15" x2 = "15.01" y1 = "9" y2 = "9" ></ line > < / > } } pub const LUCIDE_MEH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;