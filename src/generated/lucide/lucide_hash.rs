use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" y1 = "9" y2 = "9" x2 = "20" ></ line > < line x1 = "4" x2 = "20" y1 = "15" y2 = "15" ></ line > < line y1 = "3" x2 = "8" x1 = "10" y2 = "21" ></ line > < line x2 = "14" x1 = "16" y1 = "3" y2 = "21" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;