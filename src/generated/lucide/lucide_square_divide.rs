use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" rx = "2" ry = "2" y = "3" ></ rect > < line x1 = "8" y2 = "12" x2 = "16" y1 = "12" ></ line > < line x2 = "12" x1 = "12" y2 = "16" y1 = "16" ></ line > < line x2 = "12" y2 = "8" x1 = "12" y1 = "8" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;