use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" rx = "2" ry = "2" width = "18" height = "18" ></ rect > < line y2 = "9" x2 = "21" x1 = "3" y1 = "9" ></ line > < line x1 = "3" y1 = "15" x2 = "21" y2 = "15" ></ line > < line x2 = "9" x1 = "9" y1 = "9" y2 = "21" ></ line > < line x1 = "15" x2 = "15" y2 = "21" y1 = "9" ></ line > < / > } } pub const LucideSheet : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;