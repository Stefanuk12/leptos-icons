use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" y1 = "6" y2 = "6" x2 = "21" ></ line > < line x2 = "21" x1 = "3" y2 = "12" y1 = "12" ></ line > < line x1 = "3" y1 = "18" y2 = "18" x2 = "21" ></ line > < / > } } pub const LUCIDE_ALIGN_JUSTIFY : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;