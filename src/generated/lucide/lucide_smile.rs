use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < path d = "M8 14s1.5 2 4 2 4-2 4-2" ></ path > < line x2 = "9.01" y1 = "9" x1 = "9" y2 = "9" ></ line > < line x1 = "15" y1 = "9" y2 = "9" x2 = "15.01" ></ line > < / > } } pub const LUCIDE_SMILE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;