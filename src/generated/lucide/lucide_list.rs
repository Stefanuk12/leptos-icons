use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "21" x1 = "8" y1 = "6" y2 = "6" ></ line > < line x1 = "8" x2 = "21" y1 = "12" y2 = "12" ></ line > < line y1 = "18" y2 = "18" x2 = "21" x1 = "8" ></ line > < line x1 = "3" y1 = "6" y2 = "6" x2 = "3.01" ></ line > < line x1 = "3" y1 = "12" y2 = "12" x2 = "3.01" ></ line > < line y1 = "18" x2 = "3.01" y2 = "18" x1 = "3" ></ line > < / > } } pub const LUCIDE_LIST : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;