use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "9" y1 = "9" x2 = "20" x1 = "4" ></ line > < line y1 = "15" y2 = "15" x2 = "20" x1 = "4" ></ line > < line y2 = "21" x1 = "10" x2 = "8" y1 = "3" ></ line > < line x1 = "16" x2 = "14" y1 = "3" y2 = "21" ></ line > < / > } } pub const LUCIDE_HASH : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;