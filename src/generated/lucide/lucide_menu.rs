use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "20" y2 = "12" y1 = "12" x1 = "4" ></ line > < line y1 = "6" x1 = "4" y2 = "6" x2 = "20" ></ line > < line x1 = "4" y1 = "18" x2 = "20" y2 = "18" ></ line > < / > } } pub const LUCIDE_MENU : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;