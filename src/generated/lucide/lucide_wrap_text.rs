use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "21" y2 = "6" x1 = "3" y1 = "6" ></ line > < path d = "M3 12h15a3 3 0 1 1 0 6h-4" ></ path > < polyline points = "16 16 14 18 16 20" ></ polyline > < line y1 = "18" y2 = "18" x1 = "3" x2 = "10" ></ line > < / > } } pub const LUCIDE_WRAP_TEXT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;