use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" rx = "2" y = "3" x = "2" height = "14" ></ rect > < line x1 = "8" y2 = "21" y1 = "21" x2 = "16" ></ line > < line x1 = "12" y2 = "21" y1 = "17" x2 = "12" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;