use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "14" y = "3" x = "2" rx = "2" ></ rect > < line y2 = "21" y1 = "21" x1 = "8" x2 = "16" ></ line > < line x1 = "12" y2 = "21" x2 = "12" y1 = "17" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;