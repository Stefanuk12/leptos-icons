use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "19 20 9 12 19 4 19 20" ></ polygon > < line y2 = "5" x2 = "5" x1 = "5" y1 = "19" ></ line > < / > } } pub const LucideSkipBack : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24")] } ;