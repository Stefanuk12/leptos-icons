use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" height = "14" width = "20" y = "5" rx = "2" ></ rect > < line x2 = "22" y1 = "10" x1 = "2" y2 = "10" ></ line > < / > } } pub const LUCIDE_CREDIT_CARD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;