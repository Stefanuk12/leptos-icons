use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "19" y1 = "4" y2 = "4" x2 = "10" ></ line > < line x1 = "14" y2 = "20" y1 = "20" x2 = "5" ></ line > < line y2 = "20" x1 = "15" y1 = "4" x2 = "9" ></ line > < / > } } pub const LucideItalic : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;