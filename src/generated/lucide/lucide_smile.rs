use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < path d = "M8 14s1.5 2 4 2 4-2 4-2" ></ path > < line y2 = "9" x2 = "9.01" y1 = "9" x1 = "9" ></ line > < line y1 = "9" x1 = "15" x2 = "15.01" y2 = "9" ></ line > < / > } } pub const LUCIDE_SMILE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;