use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "6" x2 = "3" y2 = "6" x1 = "21" ></ line > < line x2 = "3" x1 = "15" y2 = "12" y1 = "12" ></ line > < line y2 = "18" y1 = "18" x1 = "17" x2 = "3" ></ line > < / > } } pub const LUCIDE_ALIGN_LEFT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;