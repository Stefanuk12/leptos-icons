use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" width = "18" x = "3" y = "3" rx = "2" height = "18" ></ rect > < line y1 = "12" x1 = "8" x2 = "16" y2 = "12" ></ line > < line y1 = "16" x1 = "12" y2 = "16" x2 = "12" ></ line > < line y2 = "8" y1 = "8" x1 = "12" x2 = "12" ></ line > < / > } } pub const LUCIDE_SQUARE_DIVIDE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;