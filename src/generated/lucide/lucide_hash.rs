use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "9" x2 = "20" y1 = "9" x1 = "4" ></ line > < line y1 = "15" x2 = "20" x1 = "4" y2 = "15" ></ line > < line x1 = "10" y2 = "21" y1 = "3" x2 = "8" ></ line > < line x1 = "16" y1 = "3" x2 = "14" y2 = "21" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;