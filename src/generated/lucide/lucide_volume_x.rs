use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "11 5 6 9 2 9 2 15 6 15 11 19 11 5" ></ polygon > < line x1 = "22" x2 = "16" y2 = "15" y1 = "9" ></ line > < line x1 = "16" y2 = "15" x2 = "22" y1 = "9" ></ line > < / > } } pub const LUCIDE_VOLUME_X : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;