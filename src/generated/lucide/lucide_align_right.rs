use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "3" y1 = "6" y2 = "6" x1 = "21" ></ line > < line y2 = "12" x1 = "21" y1 = "12" x2 = "9" ></ line > < line x1 = "21" x2 = "7" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_RIGHT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none")] } ;