use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "9" x2 = "20" x1 = "4" y1 = "9" ></ line > < line x2 = "20" y2 = "15" x1 = "4" y1 = "15" ></ line > < line x1 = "10" x2 = "8" y1 = "3" y2 = "21" ></ line > < line y1 = "3" x2 = "14" x1 = "16" y2 = "21" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;