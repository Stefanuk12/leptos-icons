use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" ry = "2" width = "16" rx = "2" x = "4" height = "20" ></ rect > < line x1 = "12" x2 = "12.01" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_TABLET : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;