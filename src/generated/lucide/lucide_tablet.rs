use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "4" y = "2" ry = "2" rx = "2" height = "20" width = "16" ></ rect > < line y2 = "18" y1 = "18" x1 = "12" x2 = "12.01" ></ line > < / > } } pub const LUCIDE_TABLET : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;