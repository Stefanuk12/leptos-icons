use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "14" y2 = "2" y1 = "2" x1 = "10" ></ line > < line y1 = "14" x2 = "15" x1 = "12" y2 = "11" ></ line > < circle r = "8" cy = "14" cx = "12" ></ circle > < / > } } pub const LUCIDE_TIMER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none")] } ;