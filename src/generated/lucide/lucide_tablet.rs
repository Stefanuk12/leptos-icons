use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" width = "16" height = "20" ry = "2" x = "4" rx = "2" ></ rect > < line x1 = "12" x2 = "12.01" y2 = "18" y1 = "18" ></ line > < / > } } pub const LUCIDE_TABLET : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;