use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "5" rx = "2" width = "20" height = "14" x = "2" ></ rect > < line x2 = "22" x1 = "2" y2 = "10" y1 = "10" ></ line > < / > } } pub const LUCIDE_CREDIT_CARD : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;