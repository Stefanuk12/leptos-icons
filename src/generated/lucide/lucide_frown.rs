use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < path d = "M16 16s-1.5-2-4-2-4 2-4 2" ></ path > < line x1 = "9" y1 = "9" y2 = "9" x2 = "9.01" ></ line > < line y1 = "9" y2 = "9" x2 = "15.01" x1 = "15" ></ line > < / > } } pub const LUCIDE_FROWN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24")] } ;