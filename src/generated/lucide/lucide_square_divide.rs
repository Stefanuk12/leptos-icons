use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" ry = "2" width = "18" y = "3" height = "18" x = "3" ></ rect > < line y1 = "12" x2 = "16" y2 = "12" x1 = "8" ></ line > < line y1 = "16" x2 = "12" y2 = "16" x1 = "12" ></ line > < line x1 = "12" x2 = "12" y1 = "8" y2 = "8" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;