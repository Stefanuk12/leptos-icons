use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "8" x2 = "21" y1 = "6" y2 = "6" ></ line > < line y1 = "12" x1 = "8" x2 = "21" y2 = "12" ></ line > < line x1 = "8" x2 = "21" y1 = "18" y2 = "18" ></ line > < line y2 = "6" x2 = "3.01" y1 = "6" x1 = "3" ></ line > < line y2 = "12" x1 = "3" y1 = "12" x2 = "3.01" ></ line > < line x1 = "3" y2 = "18" x2 = "3.01" y1 = "18" ></ line > < / > } } pub const LUCIDE_LIST : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;