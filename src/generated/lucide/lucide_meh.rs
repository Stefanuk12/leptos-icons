use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < line y2 = "15" x2 = "16" y1 = "15" x1 = "8" ></ line > < line y2 = "9" x2 = "9.01" x1 = "9" y1 = "9" ></ line > < line x2 = "15.01" x1 = "15" y1 = "9" y2 = "9" ></ line > < / > } } pub const LUCIDE_MEH : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;