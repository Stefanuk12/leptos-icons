use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 4v6a6 6 0 0 0 12 0V4" ></ path > < line x2 = "20" y2 = "20" y1 = "20" x1 = "4" ></ line > < / > } } pub const LUCIDE_UNDERLINE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;