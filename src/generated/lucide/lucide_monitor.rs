use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" rx = "2" x = "2" y = "3" height = "14" ></ rect > < line x2 = "16" x1 = "8" y1 = "21" y2 = "21" ></ line > < line y2 = "21" x1 = "12" x2 = "12" y1 = "17" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24")] } ;