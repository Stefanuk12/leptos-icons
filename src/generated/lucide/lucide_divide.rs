use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "6" cx = "12" r = "1" ></ circle > < line y2 = "12" y1 = "12" x2 = "19" x1 = "5" ></ line > < circle cx = "12" r = "1" cy = "18" ></ circle > < / > } } pub const LUCIDE_DIVIDE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;