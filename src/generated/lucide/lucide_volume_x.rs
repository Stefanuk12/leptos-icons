use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "11 5 6 9 2 9 2 15 6 15 11 19 11 5" ></ polygon > < line y1 = "9" x2 = "16" x1 = "22" y2 = "15" ></ line > < line x1 = "16" x2 = "22" y2 = "15" y1 = "9" ></ line > < / > } } pub const LUCIDE_VOLUME_X : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none")] } ;