use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" x2 = "20" y2 = "12" y1 = "12" ></ line > < line x1 = "4" x2 = "20" y2 = "6" y1 = "6" ></ line > < line x2 = "20" y1 = "18" x1 = "4" y2 = "18" ></ line > < / > } } pub const LUCIDE_MENU : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;