use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "12" y1 = "3" x2 = "12" y2 = "21" ></ line > < polyline points = "8 8 4 12 8 16" ></ polyline > < polyline points = "16 16 20 12 16 8" ></ polyline > < / > } } pub const LUCIDE_SEPARATOR_VERTICAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24")] } ;