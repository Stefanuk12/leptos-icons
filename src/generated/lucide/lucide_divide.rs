use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "1" cy = "6" ></ circle > < line y2 = "12" x1 = "5" x2 = "19" y1 = "12" ></ line > < circle r = "1" cx = "12" cy = "18" ></ circle > < / > } } pub const LUCIDE_DIVIDE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;