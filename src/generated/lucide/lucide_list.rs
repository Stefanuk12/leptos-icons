use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "8" x2 = "21" y2 = "6" y1 = "6" ></ line > < line y1 = "12" y2 = "12" x1 = "8" x2 = "21" ></ line > < line x1 = "8" y2 = "18" y1 = "18" x2 = "21" ></ line > < line x2 = "3.01" y2 = "6" x1 = "3" y1 = "6" ></ line > < line x2 = "3.01" x1 = "3" y1 = "12" y2 = "12" ></ line > < line y1 = "18" y2 = "18" x2 = "3.01" x1 = "3" ></ line > < / > } } pub const LUCIDE_LIST : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;