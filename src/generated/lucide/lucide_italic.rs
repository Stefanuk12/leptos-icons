use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "19" x2 = "10" y2 = "4" y1 = "4" ></ line > < line x2 = "5" x1 = "14" y1 = "20" y2 = "20" ></ line > < line x1 = "15" y1 = "4" x2 = "9" y2 = "20" ></ line > < / > } } pub const LucideItalic : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;