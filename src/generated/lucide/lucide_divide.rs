use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "6" r = "1" cx = "12" ></ circle > < line x2 = "19" y2 = "12" y1 = "12" x1 = "5" ></ line > < circle cy = "18" r = "1" cx = "12" ></ circle > < / > } } pub const LUCIDE_DIVIDE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;