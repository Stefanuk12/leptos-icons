use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" y1 = "9" y2 = "9" x2 = "20" ></ line > < line y1 = "15" x1 = "4" y2 = "15" x2 = "20" ></ line > < line x2 = "8" y1 = "3" y2 = "21" x1 = "10" ></ line > < line y1 = "3" x1 = "16" y2 = "21" x2 = "14" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;