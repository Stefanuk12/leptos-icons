use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < path d = "M8 14s1.5 2 4 2 4-2 4-2" ></ path > < line y2 = "9" y1 = "9" x1 = "9" x2 = "9.01" ></ line > < line y2 = "9" y1 = "9" x1 = "15" x2 = "15.01" ></ line > < / > } } pub const LUCIDE_SMILE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;