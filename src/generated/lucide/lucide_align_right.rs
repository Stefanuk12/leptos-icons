use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" x2 = "3" x1 = "21" y1 = "6" ></ line > < line x1 = "21" y1 = "12" y2 = "12" x2 = "9" ></ line > < line y1 = "18" y2 = "18" x2 = "7" x1 = "21" ></ line > < / > } } pub const LUCIDE_ALIGN_RIGHT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;