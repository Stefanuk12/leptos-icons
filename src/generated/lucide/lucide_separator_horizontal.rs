use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" x2 = "21" y2 = "12" y1 = "12" ></ line > < polyline points = "8 8 12 4 16 8" ></ polyline > < polyline points = "16 16 12 20 8 16" ></ polyline > < / > } } pub const LUCIDE_SEPARATOR_HORIZONTAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;