use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < line y2 = "15" x2 = "16" y1 = "15" x1 = "8" ></ line > < line y1 = "9" x1 = "9" y2 = "9" x2 = "9.01" ></ line > < line x1 = "15" x2 = "15.01" y2 = "9" y1 = "9" ></ line > < / > } } pub const LUCIDE_MEH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;