use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "9" x2 = "20" x1 = "4" y2 = "9" ></ line > < line y1 = "15" x1 = "4" x2 = "20" y2 = "15" ></ line > < line x1 = "10" x2 = "8" y2 = "21" y1 = "3" ></ line > < line x1 = "16" y1 = "3" x2 = "14" y2 = "21" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;