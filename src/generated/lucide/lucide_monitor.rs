use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "14" y = "3" rx = "2" x = "2" width = "20" ></ rect > < line x1 = "8" x2 = "16" y1 = "21" y2 = "21" ></ line > < line x2 = "12" x1 = "12" y2 = "21" y1 = "17" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;