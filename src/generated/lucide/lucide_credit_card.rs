use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" y = "5" width = "20" rx = "2" height = "14" ></ rect > < line x1 = "2" x2 = "22" y2 = "10" y1 = "10" ></ line > < / > } } pub const LUCIDE_CREDIT_CARD : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none")] } ;