use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" x1 = "8" x2 = "16" y1 = "12" ></ line > < line y1 = "16" x2 = "12" x1 = "12" y2 = "16" ></ line > < line y1 = "8" x2 = "12" x1 = "12" y2 = "8" ></ line > < circle cy = "12" r = "10" cx = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DIVIDE : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;