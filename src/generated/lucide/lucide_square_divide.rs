use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" rx = "2" ry = "2" y = "3" height = "18" ></ rect > < line y2 = "12" x1 = "8" y1 = "12" x2 = "16" ></ line > < line x1 = "12" y2 = "16" x2 = "12" y1 = "16" ></ line > < line y2 = "8" x2 = "12" x1 = "12" y1 = "8" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;