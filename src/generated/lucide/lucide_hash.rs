use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "20" y1 = "9" y2 = "9" x1 = "4" ></ line > < line x2 = "20" y2 = "15" y1 = "15" x1 = "4" ></ line > < line y2 = "21" y1 = "3" x1 = "10" x2 = "8" ></ line > < line y2 = "21" y1 = "3" x2 = "14" x1 = "16" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;