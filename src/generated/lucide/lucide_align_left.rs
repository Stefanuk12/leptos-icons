use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "6" x1 = "21" x2 = "3" y2 = "6" ></ line > < line x1 = "15" x2 = "3" y2 = "12" y1 = "12" ></ line > < line y2 = "18" y1 = "18" x2 = "3" x1 = "17" ></ line > < / > } } pub const LUCIDE_ALIGN_LEFT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;