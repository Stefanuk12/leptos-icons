use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "8" y1 = "6" y2 = "6" x2 = "21" ></ line > < line y2 = "12" y1 = "12" x2 = "21" x1 = "8" ></ line > < line x1 = "8" y2 = "18" y1 = "18" x2 = "21" ></ line > < line x1 = "3" x2 = "3.01" y2 = "6" y1 = "6" ></ line > < line x2 = "3.01" y2 = "12" x1 = "3" y1 = "12" ></ line > < line x1 = "3" y1 = "18" y2 = "18" x2 = "3.01" ></ line > < / > } } pub const LUCIDE_LIST : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24")] } ;