use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" x = "2" height = "14" y = "3" rx = "2" ></ rect > < line x2 = "16" y1 = "21" x1 = "8" y2 = "21" ></ line > < line x1 = "12" x2 = "12" y1 = "17" y2 = "21" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor")] } ;