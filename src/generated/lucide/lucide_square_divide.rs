use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" ry = "2" rx = "2" y = "3" height = "18" ></ rect > < line y2 = "12" x2 = "16" y1 = "12" x1 = "8" ></ line > < line x2 = "12" x1 = "12" y1 = "16" y2 = "16" ></ line > < line y1 = "8" x2 = "12" x1 = "12" y2 = "8" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;