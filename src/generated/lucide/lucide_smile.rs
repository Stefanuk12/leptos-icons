use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < path d = "M8 14s1.5 2 4 2 4-2 4-2" ></ path > < line x2 = "9.01" y2 = "9" y1 = "9" x1 = "9" ></ line > < line x1 = "15" y1 = "9" y2 = "9" x2 = "15.01" ></ line > < / > } } pub const LUCIDE_SMILE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;