use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 4v6a6 6 0 0 0 12 0V4" ></ path > < line y2 = "20" x1 = "4" x2 = "20" y1 = "20" ></ line > < / > } } pub const LUCIDE_UNDERLINE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;