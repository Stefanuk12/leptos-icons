use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" y = "2" width = "16" rx = "2" x = "4" height = "20" ></ rect > < line x1 = "12" x2 = "12.01" y2 = "18" y1 = "18" ></ line > < / > } } pub const LUCIDE_TABLET : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;