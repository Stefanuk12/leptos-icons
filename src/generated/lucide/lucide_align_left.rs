use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "6" x2 = "3" x1 = "21" y2 = "6" ></ line > < line x1 = "15" x2 = "3" y1 = "12" y2 = "12" ></ line > < line x2 = "3" y2 = "18" y1 = "18" x1 = "17" ></ line > < / > } } pub const LUCIDE_ALIGN_LEFT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;