use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "4" x1 = "19" x2 = "10" y2 = "4" ></ line > < line x1 = "14" y2 = "20" x2 = "5" y1 = "20" ></ line > < line y2 = "20" x2 = "9" y1 = "4" x1 = "15" ></ line > < / > } } pub const LucideItalic : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;