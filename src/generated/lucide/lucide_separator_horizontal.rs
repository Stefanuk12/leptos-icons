use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" y1 = "12" y2 = "12" x2 = "21" ></ line > < polyline points = "8 8 12 4 16 8" ></ polyline > < polyline points = "16 16 12 20 8 16" ></ polyline > < / > } } pub const LUCIDE_SEPARATOR_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("fill" , "none")] } ;