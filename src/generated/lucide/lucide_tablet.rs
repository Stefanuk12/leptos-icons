use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" height = "20" rx = "2" ry = "2" y = "2" x = "4" ></ rect > < line x1 = "12" y1 = "18" x2 = "12.01" y2 = "18" ></ line > < / > } } pub const LucideTablet : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;