use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" x2 = "16" x1 = "8" y1 = "12" ></ line > < line y2 = "16" x2 = "12" y1 = "16" x1 = "12" ></ line > < line y2 = "8" x1 = "12" y1 = "8" x2 = "12" ></ line > < circle cx = "12" r = "10" cy = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;