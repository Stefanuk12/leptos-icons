use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "21" y2 = "6" x1 = "3" y1 = "6" ></ line > < line x1 = "3" x2 = "21" y1 = "12" y2 = "12" ></ line > < line x2 = "21" y2 = "18" y1 = "18" x1 = "3" ></ line > < / > } } pub const LUCIDE_ALIGN_JUSTIFY : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;