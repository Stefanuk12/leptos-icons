use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" y = "2" height = "20" x = "4" rx = "2" ry = "2" ></ rect > < line x1 = "12" y1 = "18" y2 = "18" x2 = "12.01" ></ line > < / > } } pub const LUCIDE_TABLET : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;