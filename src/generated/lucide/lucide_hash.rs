use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "9" y1 = "9" x1 = "4" x2 = "20" ></ line > < line y2 = "15" x1 = "4" x2 = "20" y1 = "15" ></ line > < line y1 = "3" y2 = "21" x2 = "8" x1 = "10" ></ line > < line x2 = "14" y2 = "21" x1 = "16" y1 = "3" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;