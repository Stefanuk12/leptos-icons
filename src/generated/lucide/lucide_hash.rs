use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "9" x1 = "4" x2 = "20" y2 = "9" ></ line > < line x1 = "4" x2 = "20" y2 = "15" y1 = "15" ></ line > < line y2 = "21" x1 = "10" x2 = "8" y1 = "3" ></ line > < line x1 = "16" x2 = "14" y2 = "21" y1 = "3" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;