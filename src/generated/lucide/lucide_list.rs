use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "6" x1 = "8" y2 = "6" x2 = "21" ></ line > < line x2 = "21" y2 = "12" x1 = "8" y1 = "12" ></ line > < line y1 = "18" x1 = "8" y2 = "18" x2 = "21" ></ line > < line x1 = "3" x2 = "3.01" y1 = "6" y2 = "6" ></ line > < line y1 = "12" x2 = "3.01" y2 = "12" x1 = "3" ></ line > < line x2 = "3.01" x1 = "3" y2 = "18" y1 = "18" ></ line > < / > } } pub const LUCIDE_LIST : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;