use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 12h15a3 3 0 1 1 0 6h-4" ></ path > < polyline points = "16 16 14 18 16 20" ></ polyline > < line y1 = "18" x1 = "3" y2 = "18" x2 = "10" ></ line > < / > } } pub const LucideWrapText : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;