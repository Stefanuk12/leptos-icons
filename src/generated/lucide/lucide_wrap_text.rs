use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" y1 = "6" x2 = "21" y2 = "6" ></ line > < path d = "M3 12h15a3 3 0 1 1 0 6h-4" ></ path > < polyline points = "16 16 14 18 16 20" ></ polyline > < line y2 = "18" x1 = "3" y1 = "18" x2 = "10" ></ line > < / > } } pub const LUCIDE_WRAP_TEXT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;