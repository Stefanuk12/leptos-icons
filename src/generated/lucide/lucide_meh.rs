use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < line x1 = "8" x2 = "16" y2 = "15" y1 = "15" ></ line > < line x2 = "9.01" y1 = "9" x1 = "9" y2 = "9" ></ line > < line x1 = "15" y2 = "9" y1 = "9" x2 = "15.01" ></ line > < / > } } pub const LUCIDE_MEH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;