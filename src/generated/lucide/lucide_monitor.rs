use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "14" x = "2" width = "20" y = "3" rx = "2" ></ rect > < line x2 = "16" y1 = "21" y2 = "21" x1 = "8" ></ line > < line y1 = "17" y2 = "21" x1 = "12" x2 = "12" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24")] } ;