use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" x = "4" rx = "2" y = "2" width = "16" ry = "2" ></ rect > < line x1 = "12" y2 = "18" y1 = "18" x2 = "12.01" ></ line > < / > } } pub const LUCIDE_TABLET : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;