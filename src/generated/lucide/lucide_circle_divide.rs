use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" x2 = "16" x1 = "8" y1 = "12" ></ line > < line x1 = "12" y1 = "16" y2 = "16" x2 = "12" ></ line > < line x1 = "12" x2 = "12" y1 = "8" y2 = "8" ></ line > < circle cy = "12" cx = "12" r = "10" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;