use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" y2 = "12" x1 = "8" x2 = "16" ></ line > < line x2 = "12" y2 = "16" x1 = "12" y1 = "16" ></ line > < line x2 = "12" y1 = "8" x1 = "12" y2 = "8" ></ line > < circle cy = "12" cx = "12" r = "10" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DIVIDE : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;