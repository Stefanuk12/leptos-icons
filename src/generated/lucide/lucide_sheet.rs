use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" y = "3" x = "3" rx = "2" ry = "2" width = "18" ></ rect > < line y2 = "9" y1 = "9" x1 = "3" x2 = "21" ></ line > < line x1 = "3" x2 = "21" y2 = "15" y1 = "15" ></ line > < line x2 = "9" y2 = "21" x1 = "9" y1 = "9" ></ line > < line y2 = "21" x2 = "15" x1 = "15" y1 = "9" ></ line > < / > } } pub const LucideSheet : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;