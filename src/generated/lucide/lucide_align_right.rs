use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "6" x2 = "3" y2 = "6" x1 = "21" ></ line > < line x1 = "21" x2 = "9" y2 = "12" y1 = "12" ></ line > < line x2 = "7" x1 = "21" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_ALIGN_RIGHT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none")] } ;