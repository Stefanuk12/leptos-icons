use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" y = "3" rx = "2" width = "20" height = "14" ></ rect > < line x1 = "8" x2 = "16" y1 = "21" y2 = "21" ></ line > < line x1 = "12" y2 = "21" y1 = "17" x2 = "12" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;