use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < line x1 = "8" y2 = "15" x2 = "16" y1 = "15" ></ line > < line x1 = "9" y2 = "9" y1 = "9" x2 = "9.01" ></ line > < line x2 = "15.01" x1 = "15" y2 = "9" y1 = "9" ></ line > < / > } } pub const LUCIDE_MEH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;