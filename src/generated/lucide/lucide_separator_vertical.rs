use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "3" y2 = "21" x1 = "12" x2 = "12" ></ line > < polyline points = "8 8 4 12 8 16" ></ polyline > < polyline points = "16 16 20 12 16 8" ></ polyline > < / > } } pub const LUCIDE_SEPARATOR_VERTICAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24")] } ;