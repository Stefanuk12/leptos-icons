use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" x2 = "20" y1 = "9" y2 = "9" ></ line > < line x1 = "4" x2 = "20" y1 = "15" y2 = "15" ></ line > < line y1 = "3" y2 = "21" x2 = "8" x1 = "10" ></ line > < line x1 = "16" x2 = "14" y1 = "3" y2 = "21" ></ line > < / > } } pub const LucideHash : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;