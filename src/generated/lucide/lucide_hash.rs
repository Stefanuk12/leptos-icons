use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "20" y1 = "9" x1 = "4" y2 = "9" ></ line > < line y2 = "15" x2 = "20" y1 = "15" x1 = "4" ></ line > < line x2 = "8" x1 = "10" y2 = "21" y1 = "3" ></ line > < line x1 = "16" y1 = "3" y2 = "21" x2 = "14" ></ line > < / > } } pub const LucideHash : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;