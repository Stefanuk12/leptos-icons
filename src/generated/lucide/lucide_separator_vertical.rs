use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "21" y1 = "3" x2 = "12" x1 = "12" ></ line > < polyline points = "8 8 4 12 8 16" ></ polyline > < polyline points = "16 16 20 12 16 8" ></ polyline > < / > } } pub const LUCIDE_SEPARATOR_VERTICAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;