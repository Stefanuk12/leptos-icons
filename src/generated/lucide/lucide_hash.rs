use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "20" y1 = "9" x1 = "4" y2 = "9" ></ line > < line x1 = "4" x2 = "20" y1 = "15" y2 = "15" ></ line > < line x2 = "8" y2 = "21" y1 = "3" x1 = "10" ></ line > < line x2 = "14" x1 = "16" y2 = "21" y1 = "3" ></ line > < / > } } pub const LucideHash : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;