use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" rx = "2" width = "16" x = "4" height = "20" ry = "2" ></ rect > < line x2 = "12.01" y1 = "18" y2 = "18" x1 = "12" ></ line > < / > } } pub const LUCIDE_TABLET : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24")] } ;