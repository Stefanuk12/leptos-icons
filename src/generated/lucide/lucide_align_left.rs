use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "21" y1 = "6" x2 = "3" y2 = "6" ></ line > < line y1 = "12" x1 = "15" x2 = "3" y2 = "12" ></ line > < line x1 = "17" x2 = "3" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_LEFT : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;