use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 12h15a3 3 0 1 1 0 6h-4" ></ path > < polyline points = "16 16 14 18 16 20" ></ polyline > < line x1 = "3" x2 = "10" y2 = "18" y1 = "18" ></ line > < / > } } pub const LucideWrapText : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;