use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "4" y2 = "9" x2 = "20" y1 = "9" ></ line > < line y1 = "15" x2 = "20" y2 = "15" x1 = "4" ></ line > < line y1 = "3" x2 = "8" y2 = "21" x1 = "10" ></ line > < line x1 = "16" x2 = "14" y2 = "21" y1 = "3" ></ line > < / > } } pub const LucideHash : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;