use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "20" x1 = "4" y2 = "12" y1 = "12" ></ line > < line x1 = "4" y2 = "6" y1 = "6" x2 = "20" ></ line > < line x1 = "4" y1 = "18" x2 = "20" y2 = "18" ></ line > < / > } } pub const LUCIDE_MENU : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;