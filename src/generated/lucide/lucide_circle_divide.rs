use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "8" y2 = "12" x2 = "16" y1 = "12" ></ line > < line x1 = "12" x2 = "12" y1 = "16" y2 = "16" ></ line > < line y2 = "8" y1 = "8" x2 = "12" x1 = "12" ></ line > < circle cx = "12" r = "10" cy = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DIVIDE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;