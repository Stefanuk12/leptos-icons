use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "11 5 6 9 2 9 2 15 6 15 11 19 11 5" ></ polygon > < line y2 = "15" x1 = "22" y1 = "9" x2 = "16" ></ line > < line y1 = "9" x1 = "16" y2 = "15" x2 = "22" ></ line > < / > } } pub const LUCIDE_VOLUME_X : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;