use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" x2 = "21" y2 = "6" y1 = "6" ></ line > < line y2 = "12" y1 = "12" x1 = "3" x2 = "21" ></ line > < line x1 = "3" y1 = "18" y2 = "18" x2 = "21" ></ line > < / > } } pub const LUCIDE_ALIGN_JUSTIFY : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;