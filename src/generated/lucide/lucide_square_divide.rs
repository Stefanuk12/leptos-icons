use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" ry = "2" y = "3" width = "18" x = "3" rx = "2" ></ rect > < line y1 = "12" x1 = "8" y2 = "12" x2 = "16" ></ line > < line x1 = "12" x2 = "12" y2 = "16" y1 = "16" ></ line > < line y1 = "8" x1 = "12" y2 = "8" x2 = "12" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;