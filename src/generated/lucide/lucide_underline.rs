use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 4v6a6 6 0 0 0 12 0V4" ></ path > < line x2 = "20" y2 = "20" x1 = "4" y1 = "20" ></ line > < / > } } pub const LUCIDE_UNDERLINE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;