use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" y2 = "12" x2 = "21" y1 = "12" ></ line > < polyline points = "8 8 12 4 16 8" ></ polyline > < polyline points = "16 16 12 20 8 16" ></ polyline > < / > } } pub const LUCIDE_SEPARATOR_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;