use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "9" x1 = "4" y2 = "9" x2 = "20" ></ line > < line x1 = "4" y1 = "15" y2 = "15" x2 = "20" ></ line > < line x1 = "10" y1 = "3" y2 = "21" x2 = "8" ></ line > < line y2 = "21" x2 = "14" x1 = "16" y1 = "3" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;