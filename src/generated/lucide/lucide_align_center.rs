use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "21" y1 = "6" x2 = "3" y2 = "6" ></ line > < line x2 = "7" x1 = "17" y2 = "12" y1 = "12" ></ line > < line y1 = "18" x2 = "5" x1 = "19" y2 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_CENTER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;