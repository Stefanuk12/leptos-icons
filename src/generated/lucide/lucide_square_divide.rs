use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" rx = "2" height = "18" y = "3" ry = "2" ></ rect > < line y2 = "12" y1 = "12" x1 = "8" x2 = "16" ></ line > < line x2 = "12" y1 = "16" x1 = "12" y2 = "16" ></ line > < line x1 = "12" x2 = "12" y1 = "8" y2 = "8" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;