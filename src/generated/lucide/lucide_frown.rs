use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < path d = "M16 16s-1.5-2-4-2-4 2-4 2" ></ path > < line y1 = "9" x1 = "9" x2 = "9.01" y2 = "9" ></ line > < line x2 = "15.01" y2 = "9" y1 = "9" x1 = "15" ></ line > < / > } } pub const LUCIDE_FROWN : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;