use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "10" y1 = "12" x1 = "6" y2 = "12" ></ line > < line x2 = "8" y1 = "10" x1 = "8" y2 = "14" ></ line > < line y1 = "13" x1 = "15" x2 = "15.01" y2 = "13" ></ line > < line x2 = "18.01" x1 = "18" y1 = "11" y2 = "11" ></ line > < rect x = "2" height = "12" width = "20" rx = "2" y = "6" ></ rect > < / > } } pub const LucideGamepad : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round")] } ;