use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2" ></ polygon > < line x2 = "12" y1 = "8" y2 = "12" x1 = "12" ></ line > < line y1 = "16" x2 = "12.01" x1 = "12" y2 = "16" ></ line > < / > } } pub const LUCIDE_OCTAGON_ALERT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none")] } ;