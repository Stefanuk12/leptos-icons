use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "3" x1 = "21" y1 = "6" y2 = "6" ></ line > < line y1 = "12" x1 = "15" x2 = "3" y2 = "12" ></ line > < line x1 = "17" y1 = "18" x2 = "3" y2 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_LEFT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;