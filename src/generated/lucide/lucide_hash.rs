use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" y2 = "9" x2 = "20" y1 = "9" ></ line > < line x2 = "20" x1 = "4" y2 = "15" y1 = "15" ></ line > < line x2 = "8" y2 = "21" x1 = "10" y1 = "3" ></ line > < line x1 = "16" y1 = "3" y2 = "21" x2 = "14" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2")] } ;