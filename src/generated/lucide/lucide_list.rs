use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" x2 = "21" x1 = "8" y1 = "6" ></ line > < line x2 = "21" x1 = "8" y1 = "12" y2 = "12" ></ line > < line y1 = "18" x2 = "21" x1 = "8" y2 = "18" ></ line > < line y1 = "6" x1 = "3" x2 = "3.01" y2 = "6" ></ line > < line x2 = "3.01" x1 = "3" y1 = "12" y2 = "12" ></ line > < line x1 = "3" x2 = "3.01" y2 = "18" y1 = "18" ></ line > < / > } } pub const LUCIDE_LIST : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;