use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "20" y1 = "9" y2 = "9" x1 = "4" ></ line > < line x2 = "20" x1 = "4" y2 = "15" y1 = "15" ></ line > < line x1 = "10" x2 = "8" y1 = "3" y2 = "21" ></ line > < line y1 = "3" x1 = "16" y2 = "21" x2 = "14" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24")] } ;