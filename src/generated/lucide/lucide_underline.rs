use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 4v6a6 6 0 0 0 12 0V4" ></ path > < line y2 = "20" x2 = "20" x1 = "4" y1 = "20" ></ line > < / > } } pub const LUCIDE_UNDERLINE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;