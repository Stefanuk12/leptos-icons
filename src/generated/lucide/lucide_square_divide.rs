use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" rx = "2" height = "18" x = "3" width = "18" y = "3" ></ rect > < line x1 = "8" y1 = "12" y2 = "12" x2 = "16" ></ line > < line x2 = "12" y1 = "16" y2 = "16" x1 = "12" ></ line > < line y1 = "8" x1 = "12" x2 = "12" y2 = "8" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;