use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "4" y1 = "4" x2 = "14" x1 = "21" ></ line > < line x2 = "3" y2 = "4" x1 = "10" y1 = "4" ></ line > < line x2 = "12" y1 = "12" x1 = "21" y2 = "12" ></ line > < line y1 = "12" y2 = "12" x1 = "8" x2 = "3" ></ line > < line y1 = "20" x2 = "16" y2 = "20" x1 = "21" ></ line > < line y2 = "20" x1 = "12" x2 = "3" y1 = "20" ></ line > < line x2 = "14" x1 = "14" y1 = "2" y2 = "6" ></ line > < line x1 = "8" y2 = "14" y1 = "10" x2 = "8" ></ line > < line y1 = "18" x2 = "16" y2 = "22" x1 = "16" ></ line > < / > } } pub const LucideSlidersHorizontal : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;