use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "10" x2 = "21" y1 = "6" y2 = "6" ></ line > < line y1 = "12" x2 = "21" x1 = "10" y2 = "12" ></ line > < line x1 = "10" x2 = "21" y1 = "18" y2 = "18" ></ line > < path d = "M4 6h1v4" ></ path > < path d = "M4 10h2" ></ path > < path d = "M6 18H4c0-1 2-2 2-3s-1-1.5-2-1" ></ path > < / > } } pub const LUCIDE_LIST_ORDERED : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;