use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" x2 = "16" x1 = "8" y1 = "12" ></ line > < line y2 = "16" x1 = "12" y1 = "16" x2 = "12" ></ line > < line y1 = "8" y2 = "8" x1 = "12" x2 = "12" ></ line > < circle cx = "12" cy = "12" r = "10" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DIVIDE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round")] } ;