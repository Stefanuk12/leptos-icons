use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" y1 = "9" y2 = "9" x2 = "20" ></ line > < line x2 = "20" y2 = "15" x1 = "4" y1 = "15" ></ line > < line x2 = "8" y1 = "3" y2 = "21" x1 = "10" ></ line > < line x2 = "14" y1 = "3" y2 = "21" x1 = "16" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;