use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" y = "5" x = "2" height = "14" rx = "2" ></ rect > < line x1 = "2" y1 = "10" y2 = "10" x2 = "22" ></ line > < / > } } pub const LUCIDE_CREDIT_CARD : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;