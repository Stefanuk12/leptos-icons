use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "21" y1 = "6" y2 = "6" x2 = "3" ></ line > < line x2 = "7" y2 = "12" x1 = "17" y1 = "12" ></ line > < line x1 = "19" y1 = "18" x2 = "5" y2 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_CENTER : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2")] } ;