use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "4" y2 = "4" x1 = "21" x2 = "14" ></ line > < line y2 = "4" x1 = "10" x2 = "3" y1 = "4" ></ line > < line x1 = "21" y2 = "12" x2 = "12" y1 = "12" ></ line > < line x2 = "3" x1 = "8" y2 = "12" y1 = "12" ></ line > < line x2 = "16" y1 = "20" x1 = "21" y2 = "20" ></ line > < line x2 = "3" y2 = "20" y1 = "20" x1 = "12" ></ line > < line y2 = "6" y1 = "2" x1 = "14" x2 = "14" ></ line > < line x1 = "8" x2 = "8" y2 = "14" y1 = "10" ></ line > < line x1 = "16" x2 = "16" y2 = "22" y1 = "18" ></ line > < / > } } pub const LucideSlidersHorizontal : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;