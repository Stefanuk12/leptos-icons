use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "21" y2 = "6" x2 = "3" y1 = "6" ></ line > < line x2 = "9" x1 = "21" y1 = "12" y2 = "12" ></ line > < line y1 = "18" x1 = "21" y2 = "18" x2 = "7" ></ line > < / > } } pub const LUCIDE_ALIGN_RIGHT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor")] } ;