use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" y1 = "6" x2 = "3" x1 = "21" ></ line > < line y2 = "12" x1 = "17" y1 = "12" x2 = "7" ></ line > < line y1 = "18" x2 = "5" x1 = "19" y2 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_CENTER : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;