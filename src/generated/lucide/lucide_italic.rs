use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "4" x2 = "10" x1 = "19" y1 = "4" ></ line > < line y1 = "20" y2 = "20" x1 = "14" x2 = "5" ></ line > < line y2 = "20" x1 = "15" x2 = "9" y1 = "4" ></ line > < / > } } pub const LucideItalic : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none")] } ;