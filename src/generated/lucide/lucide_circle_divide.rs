use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x1 = "8" x2 = "16" y2 = "12" ></ line > < line x1 = "12" x2 = "12" y1 = "16" y2 = "16" ></ line > < line y1 = "8" x1 = "12" y2 = "8" x2 = "12" ></ line > < circle cx = "12" cy = "12" r = "10" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;