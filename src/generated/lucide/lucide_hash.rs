use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "9" x2 = "20" x1 = "4" y1 = "9" ></ line > < line y2 = "15" x1 = "4" y1 = "15" x2 = "20" ></ line > < line x2 = "8" y1 = "3" x1 = "10" y2 = "21" ></ line > < line x2 = "14" y2 = "21" x1 = "16" y1 = "3" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;