use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "5" rx = "2" height = "14" width = "20" x = "2" ></ rect > < line x1 = "2" x2 = "22" y1 = "10" y2 = "10" ></ line > < / > } } pub const LUCIDE_CREDIT_CARD : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;