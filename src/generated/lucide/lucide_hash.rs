use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "9" x2 = "20" y1 = "9" x1 = "4" ></ line > < line x2 = "20" y2 = "15" y1 = "15" x1 = "4" ></ line > < line x1 = "10" y2 = "21" x2 = "8" y1 = "3" ></ line > < line x1 = "16" x2 = "14" y1 = "3" y2 = "21" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;