use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" height = "20" x = "4" rx = "2" ry = "2" y = "2" ></ rect > < line x1 = "12" x2 = "12.01" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_TABLET : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;