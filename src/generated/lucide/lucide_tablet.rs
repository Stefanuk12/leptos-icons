use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" ry = "2" x = "4" width = "16" height = "20" y = "2" ></ rect > < line x2 = "12.01" y2 = "18" x1 = "12" y1 = "18" ></ line > < / > } } pub const LUCIDE_TABLET : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;