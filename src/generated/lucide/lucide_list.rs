use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "21" x1 = "8" y1 = "6" y2 = "6" ></ line > < line x1 = "8" y2 = "12" x2 = "21" y1 = "12" ></ line > < line x2 = "21" y2 = "18" y1 = "18" x1 = "8" ></ line > < line x2 = "3.01" y1 = "6" x1 = "3" y2 = "6" ></ line > < line y2 = "12" x1 = "3" x2 = "3.01" y1 = "12" ></ line > < line y1 = "18" x1 = "3" y2 = "18" x2 = "3.01" ></ line > < / > } } pub const LUCIDE_LIST : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;