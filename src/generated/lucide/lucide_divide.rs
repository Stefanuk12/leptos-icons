use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "6" r = "1" ></ circle > < line x2 = "19" y1 = "12" x1 = "5" y2 = "12" ></ line > < circle cy = "18" r = "1" cx = "12" ></ circle > < / > } } pub const LUCIDE_DIVIDE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;