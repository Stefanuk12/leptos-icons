use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "8" y2 = "6" x2 = "21" y1 = "6" ></ line > < line x2 = "21" y1 = "12" y2 = "12" x1 = "8" ></ line > < line x2 = "21" y1 = "18" x1 = "8" y2 = "18" ></ line > < line y2 = "6" y1 = "6" x1 = "3" x2 = "3.01" ></ line > < line y2 = "12" y1 = "12" x2 = "3.01" x1 = "3" ></ line > < line y2 = "18" y1 = "18" x1 = "3" x2 = "3.01" ></ line > < / > } } pub const LUCIDE_LIST : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;