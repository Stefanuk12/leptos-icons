use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" height = "14" width = "20" y = "3" rx = "2" ></ rect > < line y1 = "21" x2 = "16" y2 = "21" x1 = "8" ></ line > < line x2 = "12" y2 = "21" x1 = "12" y1 = "17" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;