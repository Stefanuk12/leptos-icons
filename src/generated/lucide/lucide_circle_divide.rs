use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" x1 = "8" y1 = "12" x2 = "16" ></ line > < line x2 = "12" y1 = "16" x1 = "12" y2 = "16" ></ line > < line y1 = "8" x2 = "12" y2 = "8" x1 = "12" ></ line > < circle r = "10" cx = "12" cy = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DIVIDE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;