use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" x = "3" y = "3" rx = "2" ry = "2" ></ rect > < line x1 = "8" x2 = "16" y2 = "12" y1 = "12" ></ line > < line y1 = "16" x2 = "12" x1 = "12" y2 = "16" ></ line > < line x2 = "12" y2 = "8" y1 = "8" x1 = "12" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;