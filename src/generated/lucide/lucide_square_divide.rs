use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" width = "18" x = "3" height = "18" ry = "2" ></ rect > < line x2 = "16" x1 = "8" y1 = "12" y2 = "12" ></ line > < line y2 = "16" x2 = "12" x1 = "12" y1 = "16" ></ line > < line x2 = "12" x1 = "12" y1 = "8" y2 = "8" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24")] } ;