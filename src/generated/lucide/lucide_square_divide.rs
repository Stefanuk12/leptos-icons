use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" x = "3" rx = "2" ry = "2" y = "3" width = "18" ></ rect > < line x2 = "16" x1 = "8" y2 = "12" y1 = "12" ></ line > < line x1 = "12" x2 = "12" y1 = "16" y2 = "16" ></ line > < line x1 = "12" x2 = "12" y2 = "8" y1 = "8" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;