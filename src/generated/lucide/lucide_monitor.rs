use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "14" y = "3" width = "20" x = "2" ></ rect > < line y1 = "21" x2 = "16" y2 = "21" x1 = "8" ></ line > < line y2 = "21" x1 = "12" y1 = "17" x2 = "12" ></ line > < / > } } pub const LUCIDE_MONITOR : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;