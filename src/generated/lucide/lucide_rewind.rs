use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "11 19 2 12 11 5 11 19" ></ polygon > < polygon points = "22 19 13 12 22 5 22 19" ></ polygon > < / > } } pub const LUCIDE_REWIND : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24")] } ;