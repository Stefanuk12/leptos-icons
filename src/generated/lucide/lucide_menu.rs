use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "20" y2 = "12" x1 = "4" y1 = "12" ></ line > < line x1 = "4" x2 = "20" y2 = "6" y1 = "6" ></ line > < line x1 = "4" x2 = "20" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_MENU : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;