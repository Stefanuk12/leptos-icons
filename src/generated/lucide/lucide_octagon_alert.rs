use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2" ></ polygon > < line y2 = "12" x2 = "12" y1 = "8" x1 = "12" ></ line > < line x1 = "12" x2 = "12.01" y2 = "16" y1 = "16" ></ line > < / > } } pub const LUCIDE_OCTAGON_ALERT : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;