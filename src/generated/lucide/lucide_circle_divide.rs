use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "8" x2 = "16" y2 = "12" y1 = "12" ></ line > < line x1 = "12" x2 = "12" y1 = "16" y2 = "16" ></ line > < line x2 = "12" x1 = "12" y1 = "8" y2 = "8" ></ line > < circle cx = "12" cy = "12" r = "10" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DIVIDE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24")] } ;