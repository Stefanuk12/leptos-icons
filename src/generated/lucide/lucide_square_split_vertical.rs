use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 8V5c0-1 1-2 2-2h10c1 0 2 1 2 2v3" ></ path > < path d = "M19 16v3c0 1-1 2-2 2H7c-1 0-2-1-2-2v-3" ></ path > < line y1 = "12" x1 = "4" y2 = "12" x2 = "20" ></ line > < / > } } pub const LUCIDE_SQUARE_SPLIT_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;