use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 8V5c0-1 1-2 2-2h10c1 0 2 1 2 2v3" ></ path > < path d = "M19 16v3c0 1-1 2-2 2H7c-1 0-2-1-2-2v-3" ></ path > < line x1 = "4" y1 = "12" x2 = "20" y2 = "12" ></ line > < / > } } pub const LUCIDE_SQUARE_SPLIT_VERTICAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;