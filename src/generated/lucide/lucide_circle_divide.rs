use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "16" y1 = "12" x1 = "8" y2 = "12" ></ line > < line x2 = "12" x1 = "12" y1 = "16" y2 = "16" ></ line > < line y2 = "8" y1 = "8" x1 = "12" x2 = "12" ></ line > < circle cx = "12" r = "10" cy = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DIVIDE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;