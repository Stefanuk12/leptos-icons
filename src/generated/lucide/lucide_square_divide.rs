use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" x = "3" width = "18" y = "3" ry = "2" ></ rect > < line x2 = "16" x1 = "8" y1 = "12" y2 = "12" ></ line > < line y2 = "16" y1 = "16" x2 = "12" x1 = "12" ></ line > < line y1 = "8" y2 = "8" x2 = "12" x1 = "12" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24")] } ;