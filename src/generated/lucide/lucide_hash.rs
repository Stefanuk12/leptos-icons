use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "9" y1 = "9" x2 = "20" x1 = "4" ></ line > < line y1 = "15" x1 = "4" x2 = "20" y2 = "15" ></ line > < line x1 = "10" x2 = "8" y1 = "3" y2 = "21" ></ line > < line x1 = "16" y1 = "3" y2 = "21" x2 = "14" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;