use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "2" ry = "2" y = "3" height = "18" width = "18" ></ rect > < line y1 = "12" y2 = "12" x1 = "8" x2 = "16" ></ line > < line x2 = "12" y1 = "16" x1 = "12" y2 = "16" ></ line > < line y1 = "8" x1 = "12" x2 = "12" y2 = "8" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round")] } ;