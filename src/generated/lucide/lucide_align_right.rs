use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "6" x2 = "3" x1 = "21" y2 = "6" ></ line > < line x1 = "21" y1 = "12" y2 = "12" x2 = "9" ></ line > < line y1 = "18" x1 = "21" x2 = "7" y2 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_RIGHT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;