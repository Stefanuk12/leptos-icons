use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "2" rx = "2" width = "20" height = "14" ></ rect > < line x1 = "8" y2 = "21" x2 = "16" y1 = "21" ></ line > < line x1 = "12" x2 = "12" y1 = "17" y2 = "21" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round")] } ;