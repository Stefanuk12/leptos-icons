use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" x2 = "21" x1 = "10" y1 = "6" ></ line > < line y1 = "12" x2 = "21" x1 = "10" y2 = "12" ></ line > < line y2 = "18" x2 = "21" x1 = "10" y1 = "18" ></ line > < path d = "M4 6h1v4" ></ path > < path d = "M4 10h2" ></ path > < path d = "M6 18H4c0-1 2-2 2-3s-1-1.5-2-1" ></ path > < / > } } pub const LUCIDE_LIST_ORDERED : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;