use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" y1 = "9" x2 = "20" y2 = "9" ></ line > < line x1 = "4" x2 = "20" y1 = "15" y2 = "15" ></ line > < line x1 = "10" y1 = "3" x2 = "8" y2 = "21" ></ line > < line x2 = "14" y1 = "3" x1 = "16" y2 = "21" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round")] } ;