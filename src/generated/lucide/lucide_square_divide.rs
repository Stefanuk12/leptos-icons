use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" x = "3" ry = "2" height = "18" width = "18" ></ rect > < line x2 = "16" y2 = "12" x1 = "8" y1 = "12" ></ line > < line y1 = "16" x2 = "12" y2 = "16" x1 = "12" ></ line > < line x1 = "12" y1 = "8" x2 = "12" y2 = "8" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24")] } ;