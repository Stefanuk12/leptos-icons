use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "3" x1 = "21" y2 = "6" y1 = "6" ></ line > < line y1 = "12" x1 = "15" y2 = "12" x2 = "3" ></ line > < line y1 = "18" x2 = "3" x1 = "17" y2 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_LEFT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24")] } ;