use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" x2 = "20" y1 = "9" y2 = "9" ></ line > < line y2 = "15" x1 = "4" x2 = "20" y1 = "15" ></ line > < line y2 = "21" y1 = "3" x2 = "8" x1 = "10" ></ line > < line y1 = "3" x1 = "16" y2 = "21" x2 = "14" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;