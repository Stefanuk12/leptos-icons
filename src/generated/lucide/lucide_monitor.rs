use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "20" x = "2" rx = "2" height = "14" ></ rect > < line x1 = "8" x2 = "16" y2 = "21" y1 = "21" ></ line > < line y1 = "17" x1 = "12" x2 = "12" y2 = "21" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24")] } ;