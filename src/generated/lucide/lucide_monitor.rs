use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "14" rx = "2" x = "2" y = "3" ></ rect > < line y1 = "21" x2 = "16" y2 = "21" x1 = "8" ></ line > < line x2 = "12" y1 = "17" x1 = "12" y2 = "21" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24")] } ;