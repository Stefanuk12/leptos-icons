use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 8h1a4 4 0 1 1 0 8h-1" ></ path > < path d = "M3 8h14v9a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4Z" ></ path > < line x1 = "6" x2 = "6" y1 = "2" y2 = "4" ></ line > < line y1 = "2" x2 = "10" x1 = "10" y2 = "4" ></ line > < line y1 = "2" x1 = "14" x2 = "14" y2 = "4" ></ line > < / > } } pub const LUCIDE_COFFEE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none")] } ;