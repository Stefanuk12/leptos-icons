use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "21" x1 = "3" y2 = "6" y1 = "6" ></ line > < line y2 = "12" x2 = "21" x1 = "3" y1 = "12" ></ line > < line y1 = "18" x1 = "3" y2 = "18" x2 = "21" ></ line > < / > } } pub const LUCIDE_ALIGN_JUSTIFY : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;