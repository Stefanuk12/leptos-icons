use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "20" y1 = "12" y2 = "12" x1 = "4" ></ line > < line x2 = "20" x1 = "4" y1 = "6" y2 = "6" ></ line > < line y2 = "18" y1 = "18" x1 = "4" x2 = "20" ></ line > < / > } } pub const LUCIDE_MENU : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;