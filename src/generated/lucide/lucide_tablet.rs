use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" ry = "2" y = "2" height = "20" x = "4" rx = "2" ></ rect > < line x2 = "12.01" y2 = "18" y1 = "18" x1 = "12" ></ line > < / > } } pub const LUCIDE_TABLET : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;