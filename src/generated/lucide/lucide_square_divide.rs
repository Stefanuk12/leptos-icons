use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" height = "18" width = "18" rx = "2" ry = "2" ></ rect > < line x1 = "8" y2 = "12" x2 = "16" y1 = "12" ></ line > < line x1 = "12" y2 = "16" x2 = "12" y1 = "16" ></ line > < line y1 = "8" y2 = "8" x1 = "12" x2 = "12" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none")] } ;