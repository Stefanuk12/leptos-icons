use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < path d = "M8 14s1.5 2 4 2 4-2 4-2" ></ path > < line x1 = "9" x2 = "9.01" y1 = "9" y2 = "9" ></ line > < line x1 = "15" x2 = "15.01" y1 = "9" y2 = "9" ></ line > < / > } } pub const LucideSmile : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;